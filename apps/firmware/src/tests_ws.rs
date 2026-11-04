use crate::ws::{accept_key, base64, check_response, write_handshake, GatewayUrl, WsError};
use crate::ws_frame::{encode_frame, parse_header, FrameHeader, OP_BINARY, OP_PING};

const KEY: &str = "dGhlIHNhbXBsZSBub25jZQ==";
const ACCEPT: &str = "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=";

#[test]
fn test_gateway_url_parsing() {
    let url = GatewayUrl::parse("wss://bulle.facile.studio/api/hardware/ws").unwrap();
    assert_eq!((url.host, url.port, url.path), ("bulle.facile.studio", 443, "/api/hardware/ws"));
    let url = GatewayUrl::parse("wss://gw.local:8443").unwrap();
    assert_eq!((url.host, url.port, url.path), ("gw.local", 8443, "/"));
    for bad in ["ws://g/x", "wss://", "wss://:443/x", "wss://g:0/", "wss://g:99999/", "wss://a b/", "wss://u@g/"] {
        assert_eq!(GatewayUrl::parse(bad), Err(WsError::BadUrl), "{bad}");
    }
}

#[test]
fn test_handshake_request_and_response() {
    assert_eq!(base64::<8>(b"Man"), "TWFu");
    assert_eq!(base64::<8>(b"Ma"), "TWE=");
    assert_eq!(base64::<8>(b"M"), "TQ==");
    assert_eq!(accept_key(KEY), ACCEPT);

    let url = GatewayUrl::parse("wss://g.example/api/hardware/ws").unwrap();
    let mut req: heapless::String<512> = heapless::String::new();
    write_handshake(&url, KEY, "bohs_tok_x", &mut req).unwrap();
    assert!(req.starts_with("GET /api/hardware/ws HTTP/1.1\r\nHost: g.example\r\n"));
    assert!(req.contains("Sec-WebSocket-Protocol: bohs.v1\r\nAuthorization: Bearer bohs_tok_x\r\n\r\n"));

    let ok = format!(
        "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nsec-websocket-accept: {ACCEPT}\r\n{}\r\n\x02",
        "Sec-WebSocket-Protocol: bohs.v1\r\n"
    );
    assert_eq!(check_response(ok.as_bytes(), KEY), Ok(Some(ok.len() - 1)));
    assert_eq!(check_response(&ok.as_bytes()[..40], KEY), Ok(None));
    let no_proto = ok.replace("Sec-WebSocket-Protocol: bohs.v1\r\n", "");
    assert_eq!(check_response(no_proto.as_bytes(), KEY), Err(WsError::Handshake));
    let unauthorized = "HTTP/1.1 401 Unauthorized\r\nContent-Length: 0\r\n\r\n";
    assert_eq!(check_response(unauthorized.as_bytes(), KEY), Err(WsError::Handshake));
    let wrong = ok.replace(ACCEPT, "AAAAAAAAAAAAAAAAAAAAAAAAAAA=");
    assert_eq!(check_response(wrong.as_bytes(), KEY), Err(WsError::Handshake));
}

#[test]
fn test_frame_encoding_and_parsing() {
    let mut out = [0u8; 1100];
    let len = encode_frame(OP_BINARY, &[1, 2, 3], [0x10, 0x20, 0x30, 0x40], &mut out).unwrap();
    assert_eq!(&out[..len], &[0x82, 0x83, 0x10, 0x20, 0x30, 0x40, 0x11, 0x22, 0x33]);
    let len = encode_frame(OP_BINARY, &[0; 644], [0; 4], &mut out).unwrap();
    assert_eq!((len, &out[..4]), (652, &[0x82, 0xFE, 0x02, 0x84][..]));
    assert_eq!(encode_frame(OP_BINARY, &[0; 1100], [0; 4], &mut out), Err(WsError::BufferTooSmall));

    let header = FrameHeader { fin: true, opcode: OP_BINARY, header_len: 4, payload_len: 644 };
    assert_eq!(parse_header(&[0x82, 0x7E, 0x02, 0x84], 1024), Ok(Some(header)));
    assert_eq!(parse_header(&[0x82, 0x7E, 0x02], 1024), Ok(None));
    assert_eq!(parse_header(&[0x89], 1024), Ok(None));
    let ping = FrameHeader { fin: true, opcode: OP_PING, header_len: 2, payload_len: 0 };
    assert_eq!(parse_header(&[0x89, 0x00], 1024), Ok(Some(ping)));
    assert_eq!(parse_header(&[0x82, 0x85], 1024), Err(WsError::Protocol));
    assert_eq!(parse_header(&[0xC2, 0x05], 1024), Err(WsError::Protocol));
    assert_eq!(parse_header(&[0x82, 0x7E, 0x04, 0x01], 1024), Err(WsError::Protocol));
    assert_eq!(parse_header(&[0x82, 0x7F, 0, 0, 0, 1, 0, 0, 0, 0], 1024), Err(WsError::Protocol));
}
