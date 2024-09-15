use clap::{crate_authors, crate_version, value_parser, Arg, Command};
use genrs_lib::{encode_key, generate_key, generate_uuid, EncodingFormat, UuidVersion};
use uuid::Uuid;

/// Enum for common key presets
pub enum KeyPreset {
    Aes128,
    Aes192,
    Aes256,
    HmacSha256,
    HmacSha512,
    Jwt256,
    Jwt512,
    ApiKey128,
    ApiKey256,
}

fn main() {
    let matches = Command::new("Key Generator")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Generates random keys, UUIDs, and encodes them in different formats or presets")
        .help_template(
            "{name} ({version}) \n- {about-with-newline}\n\
           {all-args}\n\n{author}",
        )
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .value_name("MODE")
                .value_parser(["key", "uuid"])
                .default_value("key")
                .help("Specifies the mode: 'key' for key generation, 'uuid' for UUID generation"),
        )
        .arg(
            Arg::new("preset")
                .short('p')
                .long("preset")
                .value_name("PRESET")
                .value_parser(["aes128", "aes192", "aes256", "hmac256", "hmac512", "jwt256", "jwt512", "apikey128", "apikey256"])
                .help("Specifies a preset for common keys: aes128, aes192, aes256, hmac256, hmac512, jwt256, jwt512, apikey128, apikey256"),
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .value_parser(["hex", "base64"])
                .default_value("hex")
                .help("Specifies the encoding format for keys: hex or base64 (only for key mode)"),
        )
        .arg(
            Arg::new("length")
                .short('l')
                .long("length")
                .value_name("LENGTH")
                .value_parser(value_parser!(usize))
                .default_value("32")
                .help("Specifies the key length in bytes (default: 32 bytes / 256 bits). Ignored if preset is used."),
        )
        .arg(
            Arg::new("uuid_version")
                .short('u')
                .long("uuid-version")
                .value_name("UUID_VERSION")
                .value_parser(["v1", "v3", "v4", "v5"])
                .default_value("v4")
                .help("Specifies the UUID version (only for UUID mode)"),
        )
        .arg(
            Arg::new("namespace")
                .short('n')
                .long("namespace")
                .value_name("NAMESPACE")
                .help("Specifies the UUID namespace (only for UUID V3 or V5)"),
        )
        .arg(
            Arg::new("name")
                .short('N')
                .long("name")
                .value_name("NAME")
                .help("Specifies the name for UUID V3 or V5"),
        )
        .get_matches();

    let mode = matches.get_one::<String>("mode").unwrap();

    if mode == "key" {
        if let Some(preset) = matches.get_one::<String>("preset") {
            let (length, description) = match preset.as_str() {
                "aes128" => (16, "AES-128"),
                "aes192" => (24, "AES-192"),
                "aes256" => (32, "AES-256"),
                "hmac256" => (32, "HMAC-SHA256"),
                "hmac512" => (64, "HMAC-SHA512"),
                "jwt256" => (32, "JWT-256"),
                "jwt512" => (64, "JWT-512"),
                "apikey128" => (16, "API Key 128-bit"),
                "apikey256" => (32, "API Key 256-bit"),
                _ => unreachable!("Invalid preset"),
            };

            let format = matches.get_one::<String>("format").unwrap();
            let encoding_format = match format.as_str() {
                "hex" => EncodingFormat::Hex,
                "base64" => EncodingFormat::Base64,
                _ => unreachable!("Invalid format"),
            };

            let key = generate_key(length);
            match encode_key(key, encoding_format) {
                Ok(encoded_key) => {
                    println!("Generated Key ({} preset, {} bytes): {}", description, length, encoded_key);
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
        } else {
            let format = matches.get_one::<String>("format").unwrap();
            let length: usize = *matches.get_one::<usize>("length").unwrap();
            let encoding_format = match format.as_str() {
                "hex" => EncodingFormat::Hex,
                "base64" => EncodingFormat::Base64,
                _ => unreachable!("Invalid format"),
            };

            let key = generate_key(length);
            match encode_key(key, encoding_format) {
                Ok(encoded_key) => {
                    println!(
                        "Generated Key ({} format, {} bytes): {}",
                        format, length, encoded_key
                    );
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
        }
    } else if mode == "uuid" {
        let uuid_version = matches.get_one::<String>("uuid_version").unwrap();
        let namespace = matches.get_one::<String>("namespace");
        let name = matches.get_one::<String>("name");

        let uuid_version_enum = match uuid_version.as_str() {
            "v1" => UuidVersion::V1,
            "v3" => UuidVersion::V3,
            "v4" => UuidVersion::V4,
            "v5" => UuidVersion::V5,
            _ => unreachable!("Invalid UUID version"),
        };

        let namespace_uuid = namespace.map(|ns| Uuid::parse_str(ns).expect("Invalid UUID format for namespace"));
        let uuid_result = generate_uuid(uuid_version_enum, namespace_uuid, name.map(String::as_str));

        match uuid_result {
            Ok(uuid) => {
                println!("Generated UUID (version {}): {}", uuid_version, uuid);
            }
            Err(err) => {
                eprintln!("Error generating UUID: {}", err);
            }
        }
    }
}
