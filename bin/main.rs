use clap::{value_parser, Arg, Command};
use genrs_lib::{encode_key, generate_key};

fn main() {
    let matches = Command::new("Key Generator")
        .version("1.0")
        .about("Generates random keys and encodes them in different formats")
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .value_parser(["hex", "base64"])
                .default_value("hex")
                .help("Specifies the encoding format: hex or base64"),
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
        .get_matches();

    let format = matches.get_one::<String>("format").unwrap();
    let length: usize = *matches.get_one::<usize>("length").unwrap();

    let key = generate_key(length);
    match encode_key(key, format) {
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
