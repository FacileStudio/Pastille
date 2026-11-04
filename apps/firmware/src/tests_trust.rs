use crate::trust::{AnchorVerifier, Suite};
use embedded_tls::{CertificateEntryRef, CertificateRef, TlsCipherSuite, TlsError, TlsVerifier};

const LEAF: &[u8] = include_bytes!("../certs/test/facile-chain-0.der");
const INTERMEDIATE: &[u8] = include_bytes!("../certs/test/facile-chain-1.der");
const CROSS_SIGNED_ROOT: &[u8] = include_bytes!("../certs/test/facile-chain-2.der");

fn verify(host: &str, chain: &[&'static [u8]]) -> Result<(), TlsError> {
    let mut cert = CertificateRef::with_context(&[]);
    for der in chain {
        cert.add(CertificateEntryRef::X509(der))?;
    }
    let mut verifier = AnchorVerifier::new();
    verifier.set_hostname_verification(host)?;
    let transcript = <Suite as TlsCipherSuite>::Hash::default();
    verifier.verify_certificate(&transcript, cert)
}

#[test]
fn test_gateway_chain_must_reach_a_pinned_root_and_name_the_host() {
    let full = [LEAF, INTERMEDIATE, CROSS_SIGNED_ROOT];
    assert!(verify("bulle.facile.studio", &full).is_ok());
    assert!(verify("facile.studio", &full).is_ok());
    assert!(verify("bulle.example.com", &full).is_err());
    assert!(verify("bulle.facile.studio", &[LEAF]).is_err());
    assert!(verify("bulle.facile.studio", &[LEAF, CROSS_SIGNED_ROOT]).is_err());
    assert!(verify("bulle.facile.studio", &[INTERMEDIATE, LEAF]).is_err());
}
