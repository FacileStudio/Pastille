//! Wall clock over SNTP, needed before TLS can check certificate dates.

use crate::clock::{set_wall_time, uptime_ms};
use crate::sntp::{parse_sntp_reply, sntp_request, SNTP_PACKET_BYTES};
use embassy_net::dns::DnsQueryType;
use embassy_net::udp::{PacketMetadata, UdpSocket};
use embassy_net::Stack;
use embassy_time::{with_timeout, Duration, Timer};

const NTP_SERVER: &str = "pool.ntp.org";
const NTP_PORT: u16 = 123;
const RESYNC: Duration = Duration::from_secs(3600);
const RETRY: Duration = Duration::from_secs(10);
const REPLY_TIMEOUT: Duration = Duration::from_secs(5);

async fn sync(stack: Stack<'static>) -> bool {
    let Some(&server) = stack.dns_query(NTP_SERVER, DnsQueryType::A).await.ok().as_ref().and_then(|a| a.first()) else {
        return false;
    };
    let (mut rx_meta, mut tx_meta) = ([PacketMetadata::EMPTY; 2], [PacketMetadata::EMPTY; 2]);
    let (mut rx, mut tx) = ([0u8; 128], [0u8; 128]);
    let mut socket = UdpSocket::new(stack, &mut rx_meta, &mut rx, &mut tx_meta, &mut tx);
    if socket.bind(0).is_err() || socket.send_to(&sntp_request(), (server, NTP_PORT)).await.is_err() {
        return false;
    }
    let mut reply = [0u8; SNTP_PACKET_BYTES];
    let Ok(Ok((len, _))) = with_timeout(REPLY_TIMEOUT, socket.recv_from(&mut reply)).await else {
        return false;
    };
    parse_sntp_reply(&reply[..len]).is_some_and(|unix_ms| set_wall_time(unix_ms, uptime_ms()))
}

/// Sets the wall clock whenever the network is up and refreshes it every hour.
#[embassy_executor::task]
pub async fn sntp_task(stack: Stack<'static>) {
    loop {
        stack.wait_config_up().await;
        let synced = sync(stack).await;
        Timer::after(if synced { RESYNC } else { RETRY }).await;
    }
}
