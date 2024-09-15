use clap::{value_parser, Arg, Command};
use uuid::Uuid;
use genrs_lib::{encode_key, generate_key, generate_uuid, EncodingFormat, UuidVersion};

fn main() {
    let matches = Command::new("Key Generator")
        .version("1.1")
        .about("Generates random keys, UUIDs, and encodes them in different formats")
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
                .help("Specifies the key length in bytes (default: 32 bytes / 256 bits)"),
        )
        .arg(
            Arg::new("uuid_version")
                .short('u')
                .long("uuid-version")
                .value_name("UUID_VERSION")
                .value_parser(["v1", "v3", "v4", "v5"])
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
        // Key generation mode
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
    } else if mode == "uuid" {
        // UUID generation mode
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
