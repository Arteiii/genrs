use base64::Engine;
use rand::{rngs::OsRng, RngCore};

/// Generates a random key of the given length in bytes.
pub fn generate_key(length: usize) -> Vec<u8> {
    let mut key = vec![0u8; length];
    OsRng.try_fill_bytes(&mut key).expect(
        "Failed to generate secure random bytes. \
        Ensure that the system's entropy source is available and functioning correctly.",
    );
    key
}

/// Encodes the given key into the specified format (`hex` or `base64`).
pub fn encode_key(key: Vec<u8>, format: &str) -> Result<String, String> {
    match format {
        "hex" => Ok(hex::encode(key)),
        "base64" => Ok(base64::engine::general_purpose::STANDARD.encode(key)),
        _ => Err("Unsupported encoding format".to_string()),
    }
}
