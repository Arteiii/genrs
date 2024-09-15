# genrs

`genrs` is a command-line tool for generating random keys, UUIDs, and encoding them in different formats. It supports generating custom-length keys, common key presets, and UUIDs of various versions.

## Usage

To use `genrs`, run the following command:

```sh
genrs [OPTIONS]
```

## Options

### Key Generation Mode (default)

- `-p`, `--preset <PRESET>`
  - Specifies a preset for commonly used keys.
  - Possible values: `aes128`, `aes192`, `aes256`, `hmac256`, `hmac512`, `jwt256`, `jwt512`, `apikey128`, `apikey256`
  - Example: `genrs --preset aes256`

- `-f`, `--format <FORMAT>`
  - Specifies the encoding format for the generated key.
  - Possible values: `hex`, `base64`
  - Default: `hex`

- `-l`, `--length <LENGTH>`
  - Specifies the length of the generated key in bytes.
  - Default: `32` (i.e., 256 bits)
  - Ignored if a preset is used.

### UUID Generation Mode

- `-m`, `--mode <MODE>`
  - Specifies the mode: `key` for key generation, `uuid` for UUID generation.
  - Default: `key`

- `-u`, `--uuid-version <UUID_VERSION>`
  - Specifies the UUID version.
  - Possible values: `v1`, `v3`, `v4`, `v5`
  - Default: `v4`

- `-n`, `--namespace <NAMESPACE>`
  - Specifies the namespace UUID for UUID V3 or V5.

- `-N`, `--name <NAME>`
  - Specifies the name for UUID V3 or V5.

### General Options

- `-h`, `--help`
  - Print help information about `genrs`.

- `-V`, `--version`
  - Print the version of `genrs`.

## Examples

### Key Generation

Generate a 32-byte key in hexadecimal format (default):

```sh
genrs
```

Generate a 64-byte key in base64 format:

```sh
genrs -f base64 -l 64
```

Generate a 16-byte key using the `aes128` preset:

```sh
genrs --preset aes128
```

### UUID Generation

Generate a version 4 UUID:

```sh
genrs --mode uuid
```

Generate a version 1 UUID:

```sh
genrs --mode uuid --uuid-version v1
```

Generate a version 3 UUID with a namespace and name:

```sh
genrs --mode uuid --uuid-version v3 --namespace <UUID> --name "example"
```

## Installation

To install `genrs`, you can build it from source using Cargo:

```sh
cargo build --release
```

The resulting binary will be located in the `target/release` directory.
You can move it to a directory in your `PATH` for easy access.

## License

`genrs` is licensed under the Apache License, Version 2.0. See the [LICENSE](LICENSE) file for details.

