# pretty-env-logger [![Crates.io](https://img.shields.io/crates/v/pretty_env_logger.svg)](https://crates.io/crates/pretty_env_logger) [![Crates.io](https://img.shields.io/crates/l/pretty_env_logger.svg)](https://crates.io/crates/pretty_env_logger)

A simple logger build on top off [env_logger](https://docs.rs/env_logger).
It is configured via an environment variable and writes to standard
error with nice colored output for log levels.

- [Documentation](https://docs.rs/pretty_env_logger/)

## Usage
```rust
extern crate pretty_env_logger;
#[macro_use] extern crate log;

fn main() {
    pretty_env_logger::init().unwrap();
    info!("such information");
    warn!("o_O");
    error!("much error");
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

