use hmac::{Hmac, Mac};
use sha2::Sha256;

pub fn generate(passkey: &str, message: &str) -> String {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(passkey.as_bytes()).unwrap();
    mac.update(message.as_bytes());
    let ret = mac.finalize();
    format!("{:x}", ret.into_bytes())
}

pub fn verify(passkey: &str, message: &str, expect: &str) -> bool {
    let l = crate::generate(passkey, message);
    l.eq(expect)
}
