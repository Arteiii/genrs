# genrs

`genrs` is a command-line tool for generating random keys and encoding them in different formats

## Usage

To use `genrs`, run the following command:

```sh
genrs [OPTIONS]
```

## Options

- `-f`, `--format <FORMAT>`
    - Specifies the encoding format for the generated key.
    - Possible values: `hex`, `base64`
    - Default: `hex`

- `-l`, `--length <LENGTH>`
    - Specifies the length of the generated key in bytes.
    - Default: `32` (i.e., 256 bits)

- `-h`, `--help`
    - Print help information about `genrs`.

- `-V`, `--version`
    - Print the version of `genrs`.

## Examples

Generate a 32-byte key in hexadecimal format (default):

```sh
genrs
```

Generate a 64-byte key in base64 format:

```sh
genrs -f base64 -l 64
```

Generate a 16-byte key in hexadecimal format:

```sh
genrs -l 16
```

## Installation

To install `genrs`, you can build it from source using Cargo:

```sh
cargo build --release
```

The resulting binary will be located in the `target/release` directory. 
You can move it to a directory in your `PATH`
for easy access.

## License

`genrs` is licensed under the Apache License, Version 2.0. See the [LICENSE](LICENSE) file for details.
