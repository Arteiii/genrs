//! # genrs Library
//!
//! A versatile key and UUID generation library that allows you to:
//!
//! - Generate secure random keys of arbitrary length
//! - Encode keys in either hexadecimal (`Hex`) or Base64 (`Base64`) format
//! - Generate UUIDs of any version (V1, V3, V4, V5)
//!
//! ## Example usage
//!
//! ```
//! use genrs_lib::{encode_key, generate_key, generate_uuid, EncodingFormat, UuidVersion};
//!
//! // Generate a random key
//! let key = generate_key(32);
//! let encoded_key = encode_key(key, EncodingFormat::Base64).unwrap();
//! println!("Generated and encoded key: {}", encoded_key);
//!
//! // Generate a UUID V4
//! let uuid_v4 = generate_uuid(UuidVersion::V4, None, None).unwrap();
//! println!("Generated UUID V4: {}", uuid_v4);
//! ```
//!
//! ## Features
//!
//! - **Key Generation**: Uses a cryptographically secure random number generator (CSPRNG) to generate random keys of arbitrary length.
//! - **Key Encoding**: Supports `Hex` and `Base64` encoding formats for ease of transmission and storage.
//! - **UUID Generation**: Create universally unique identifiers (UUIDs) for V1 (timestamp-based), V3 (namespace + name, MD5), V4 (random), and V5 (namespace + name, SHA-1).
//!
//! ### Referenced Libraries
//!
//! - [`rand`](https://docs.rs/rand/0.8.4/rand/) for secure random number generation.
//! - [`uuid`](https://docs.rs/uuid/0.8.2/uuid/) for UUID generation.
//! - [`hex`](https://docs.rs/hex/0.4.2/hex/) for encoding keys in hexadecimal format.
//! - [`base64`](https://docs.rs/base64/0.13.0/base64/) for encoding keys in Base64 format.

use base64::Engine;
use rand::{rngs::OsRng, Rng, RngCore};
use uuid::{Context, Timestamp, Uuid};

/// Enum to represent the encoding format for the key.
///
/// # Examples
///
/// ```
/// use genrs_lib::{encode_key, generate_key, EncodingFormat};
///
/// let key = generate_key(32);
/// let encoded_key = encode_key(key, EncodingFormat::Base64).unwrap();
/// println!("Generated and encoded key: {}", encoded_key);
/// ```
///
/// Refer to the `encode_key` function for encoding usage.
pub enum EncodingFormat {
    Hex,
    Base64,
}

/// Generates a random key of the given length in bytes.
///
/// # Examples
///
/// ```
/// use genrs_lib::generate_key;
///
/// let key = generate_key(16);
/// assert_eq!(key.len(), 16);
/// ```
///
/// This function fills a vector of the specified length with secure random bytes
/// using the system's entropy source.
///
/// # Panics
///
/// Will panic if the system's entropy source is unavailable.
///
/// Refer to the `encode_key` function for encoding the generated key.
pub fn generate_key(length: usize) -> Vec<u8> {
    let mut key = vec![0u8; length];
    OsRng.try_fill_bytes(&mut key).expect(
        "Failed to generate secure random bytes. \
        Ensure that the system's entropy source is available and functioning correctly.",
    );
    key
}

/// Encodes the given key into the specified format (`Hex` or `Base64`).
///
/// # Examples
///
/// ```
/// use genrs_lib::{encode_key, generate_key, EncodingFormat};
///
/// let key = generate_key(16);
/// let encoded_key = encode_key(key, EncodingFormat::Hex).unwrap();
/// println!("Hex encoded key: {}", encoded_key);
/// ```
///
/// # Errors
///
/// Returns an error if the format is unsupported. However, this should never happen,
/// as the format is now restricted to the `EncodingFormat` enum.
pub fn encode_key(key: Vec<u8>, format: EncodingFormat) -> Result<String, String> {
    match format {
        EncodingFormat::Hex => Ok(hex::encode(key)),
        EncodingFormat::Base64 => Ok(base64::engine::general_purpose::STANDARD.encode(key)),
    }
}

/// Enum to represent UUID versions.
///
/// # Examples
///
/// ```
/// use genrs_lib::{generate_uuid, UuidVersion};
///
/// let uuid_v4 = generate_uuid(UuidVersion::V4, None, None).unwrap();
/// println!("Generated UUID V4: {}", uuid_v4);
/// ```
///
/// Refer to the `generate_uuid` function for usage.
pub enum UuidVersion {
    V1,
    V3,
    V4,
    V5,
}

/// Generates a UUID of the specified version.
///
/// - **UUID V1**: Generates a UUID based on the current system time and a random node ID.
/// - **UUID V3 and V5**: Require a namespace and name for generating a UUID based on the MD5 or SHA-1 hash.
/// - **UUID V4**: Generates a purely random UUID.
///
/// # Examples
///
/// ```
/// use uuid::Uuid;
/// use genrs_lib::{generate_uuid, UuidVersion};
///
/// let uuid_v1 = generate_uuid(UuidVersion::V1, None, None).unwrap();
/// println!("Generated UUID V1: {}", uuid_v1);
///
/// let namespace = Uuid::new_v4();
/// let uuid_v3 = generate_uuid(UuidVersion::V3, Some(namespace), Some("example")).unwrap();
/// println!("Generated UUID V3: {}", uuid_v3);
/// ```
///
/// # Errors
///
/// Returns an error if the required parameters (namespace, name) for UUID V3 or V5 are missing.
pub fn generate_uuid(version: UuidVersion, namespace: Option<Uuid>, name: Option<&str>) -> Result<Uuid, String> {
    match version {
        UuidVersion::V1 => {
            let context = Context::new(OsRng.next_u64() as u16);
            let ts = Timestamp::now(&context);
            let node_id: [u8; 6] = OsRng.gen();

            Ok(Uuid::new_v1(ts, &node_id))
        }
        UuidVersion::V3 => {
            if let (Some(namespace), Some(name)) = (namespace, name) {
                Ok(Uuid::new_v3(&namespace, name.as_bytes()))
            } else {
                Err("Namespace and name are required for UUID V3".to_string())
            }
        }
        UuidVersion::V4 => Ok(Uuid::new_v4()),
        UuidVersion::V5 => {
            if let (Some(namespace), Some(name)) = (namespace, name) {
                Ok(Uuid::new_v5(&namespace, name.as_bytes()))
            } else {
                Err("Namespace and name are required for UUID V5".to_string())
            }
        }
    }
}
