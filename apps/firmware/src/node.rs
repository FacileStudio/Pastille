use crate::audio::AudioFrame;
use crate::avatar::AvatarRenderer;
use crate::ble::{BleProvisioner, ProvisionConfig, ProvisionState, TransportKind};
use crate::events::TapType;
use crate::link::{outbound, Inbound, AUDIO_RX_CHANNEL, AVATAR_SIGNAL, HAPTIC_SIGNAL};
use crate::power::PowerManager;
use crate::protocol::{BohsFrame, ProtocolDispatcher};
use crate::MAX_PACKET_BYTES;

/// Protocol state owned by the network task: one Reverse MCP server and one provisioner.
pub struct Node {
    pub dispatcher: ProtocolDispatcher,
    pub avatar: AvatarRenderer,
    pub prov: BleProvisioner,
    pub power: PowerManager,
    /// The link the gateway session currently uses; notifications and upstream audio go there.
    pub gateway_link: Option<TransportKind>,
    pub(crate) seq: u16,
    pub(crate) was_active: bool,
}

impl Node {
    /// Builds the node, restoring the config the device stored in flash, if any.
    pub fn new(hardware_id: [u8; 4], stored: Option<ProvisionConfig>) -> Self {
        let mut prov = BleProvisioner::new(hardware_id);
        if let Some(cfg) = stored {
            prov.restore(cfg);
        }
        let mut power = PowerManager::new();
        power.on_wake_trigger();
        power.on_hardware_ready(prov.transport);
        let (dispatcher, avatar) = (ProtocolDispatcher::new(), AvatarRenderer::new());
        Self { dispatcher, avatar, prov, power, gateway_link: None, seq: 0, was_active: false }
    }

    /// Applies one event, then publishes every side effect of a provisioning state change.
    pub fn handle(&mut self, event: Inbound) {
        let before = self.prov.state;
        let old_config = (before == ProvisionState::Authorized).then(|| self.prov.config.clone()).flatten();
        match event {
            Inbound::Frame(link, (buf, len)) => self.handle_inbound(link, &buf[..len.min(MAX_PACKET_BYTES)]),
            Inbound::BleConnected(handle) => {
                self.prov.on_connected(handle);
                self.power.on_ble_connected();
            }
            Inbound::BleSecured(bonded) => self.prov.on_link_encrypted(bonded),
            Inbound::BleDisconnected => {
                self.prov.on_disconnect();
                self.link_down(TransportKind::BleRelay);
            }
            Inbound::Tap(kind, uptime_ms) => self.on_tap(kind, uptime_ms),
            Inbound::LinkUp(link) => self.link_up(link),
            Inbound::LinkDown(link) => self.link_down(link),
            Inbound::LowBattery(pct, mv) => {
                self.send_event(|out| ProtocolDispatcher::encode_low_battery_alert(pct, mv, out));
            }
        }
        self.after_event(before, old_config);
    }

    /// Routes one frame received over `link`: audio to playback, `0x02` to the Reverse MCP server,
    /// `0x03` to the provisioner. Replies go back on the same link. Over WSS a `0x03` frame is
    /// dropped unanswered, because the device never sends `0x03` there.
    pub fn handle_inbound(&mut self, link: TransportKind, packet: &[u8]) {
        let mut out = [0u8; MAX_PACKET_BYTES];
        let written = match self.dispatcher.decode_packet(packet) {
            Ok(BohsFrame::Audio { payload, .. }) => {
                self.gateway_link = Some(link);
                let mut frame = AudioFrame::new();
                if frame.copy_from_bytes(payload).is_ok() {
                    let _ = AUDIO_RX_CHANNEL.try_send(frame);
                }
                None
            }
            Ok(BohsFrame::Control { json }) => {
                self.gateway_link = Some(link);
                self.dispatch_control(json, &mut out)
            }
            Ok(BohsFrame::Provision { .. }) if link == TransportKind::WifiDirect => None,
            Ok(BohsFrame::Provision { json }) => self.prov.dispatch_provision_rpc(json, link, &mut out),
            Err(_) => None,
        };
        if let Some(len) = written {
            let _ = outbound(link).try_send((out, len));
        }
    }

    fn dispatch_control(&mut self, json: &str, out: &mut [u8]) -> Option<usize> {
        let before = (self.avatar.state, self.avatar.emotion);
        let written = self.dispatcher.dispatch_json_rpc(json, &mut self.avatar, out);
        let after = (self.avatar.state, self.avatar.emotion);
        if after != before {
            AVATAR_SIGNAL.signal(after);
        }
        if let Some(pulse) = self.dispatcher.haptic.take() {
            HAPTIC_SIGNAL.signal(pulse);
        }
        written
    }

    /// Advances the provisioning window by `elapsed_ms` and publishes a resulting state change.
    pub fn tick(&mut self, elapsed_ms: u32) {
        let before = self.prov.state;
        let old_config = self.prov.config.clone();
        self.prov.tick(elapsed_ms);
        self.after_event(before, old_config);
    }

    fn on_tap(&mut self, kind: TapType, uptime_ms: u64) {
        self.power.on_wake_trigger();
        self.prov.on_physical_tap();
        self.send_event(|out| ProtocolDispatcher::encode_tap_event(kind, uptime_ms, out));
    }

    fn link_up(&mut self, link: TransportKind) {
        self.gateway_link = Some(link);
        if link == TransportKind::WifiDirect {
            self.power.on_ws_connected();
        }
    }

    fn link_down(&mut self, link: TransportKind) {
        if self.gateway_link == Some(link) {
            self.gateway_link = None;
        }
    }
}
