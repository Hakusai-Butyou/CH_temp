use rand::Rng;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

pub fn generate_session_id() -> String {
    let mut bytes = [0u8; 32];  // 256bit
    rand::rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}