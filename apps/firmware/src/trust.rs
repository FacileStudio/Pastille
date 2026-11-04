//! TLS 1.3 trust for the gateway connection: the server chain must end at one of four pinned
//! root CAs, name the gateway host, and be valid at the current wall time. The WSS task only
//! opens a session once SNTP has set the clock, so the dates are always checked on the device.

use embedded_tls::pki::CertVerifier;
use embedded_tls::{Aes128GcmSha256, Certificate, CertificateEntryRef, CertificateRef, CertificateVerifyRef};
use crate::clock::WallClock;
use embedded_tls::{TlsCipherSuite, TlsError, TlsVerifier};

/// Root CAs the gateway chain may end at: Let's Encrypt and Google Trust Services, with the
/// GlobalSign root that cross-signs GTS Root R4.
const ANCHORS: [&[u8]; 4] = [
    include_bytes!("../certs/isrg-root-x1.der"),
    include_bytes!("../certs/isrg-root-x2.der"),
    include_bytes!("../certs/gts-root-r4.der"),
    include_bytes!("../certs/globalsign-root-r1.der"),
];
/// Room for the server leaf certificate kept for the CertificateVerify step.
const CERT_BYTES: usize = 4096;

/// The one TLS 1.3 cipher suite the device offers.
pub type Suite = Aes128GcmSha256;
type Verifier = CertVerifier<'static, Suite, WallClock, CERT_BYTES>;

/// Tries each pinned root in turn and keeps the verifier that accepted the chain.
pub struct AnchorVerifier {
    host: heapless::String<64>,
    chosen: Option<Verifier>,
}

impl AnchorVerifier {
    pub const fn new() -> Self {
        Self { host: heapless::String::new(), chosen: None }
    }
}

fn copy_chain<'a>(cert: &CertificateRef<'a>) -> Result<CertificateRef<'a>, TlsError> {
    let mut copy = CertificateRef::with_context(&[]);
    for entry in &cert.entries {
        copy.add(match entry {
            CertificateEntryRef::X509(der) => CertificateEntryRef::X509(der),
            CertificateEntryRef::RawPublicKey(key) => CertificateEntryRef::RawPublicKey(key),
        })?;
    }
    Ok(copy)
}

impl TlsVerifier<Suite> for AnchorVerifier {
    fn set_hostname_verification(&mut self, hostname: &str) -> Result<(), TlsError> {
        self.host = heapless::String::try_from(hostname).map_err(|_| TlsError::InsufficientSpace)?;
        Ok(())
    }

    fn verify_certificate(
        &mut self,
        transcript: &<Suite as TlsCipherSuite>::Hash,
        cert: CertificateRef,
    ) -> Result<(), TlsError> {
        for anchor in ANCHORS {
            let mut verifier = Verifier::new(Certificate::X509(anchor));
            verifier.set_hostname_verification(&self.host)?;
            if verifier.verify_certificate(transcript, copy_chain(&cert)?).is_ok() {
                self.chosen = Some(verifier);
                return Ok(());
            }
        }
        Err(TlsError::InvalidCertificate)
    }

    fn verify_signature(&mut self, verify: CertificateVerifyRef) -> Result<(), TlsError> {
        self.chosen.as_mut().ok_or(TlsError::InvalidCertificate)?.verify_signature(verify)
    }
}

