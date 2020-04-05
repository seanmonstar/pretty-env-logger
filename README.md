# pretty-env-logger

[![Crates.io](https://img.shields.io/crates/v/pretty_env_logger.svg)](https://crates.io/crates/pretty_env_logger)
[![Docs](https://docs.rs/pretty_env_logger/badge.svg)](https://docs.rs/pretty_env_logger)
[![MIT/APACHE-2.0](https://img.shields.io/crates/l/pretty_env_logger.svg)](https://crates.io/crates/pretty_env_logger)
[![Travis CI](https://travis-ci.org/seanmonstar/pretty-env-logger.svg?branch=master)](https://travis-ci.org/seanmonstar/pretty-env-logger)

A simple logger built on top of [env_logger](https://docs.rs/env_logger).
It is configured via an environment variable and writes to standard
error with nice colored output for log levels.

![example output](readme-example.png)

## Usage

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
log = "0.4"
pretty_env_logger = "0.4"
```

Add some usage to your application:

```rust
extern crate pretty_env_logger;
#[macro_use] extern crate log;

fn main() {
    pretty_env_logger::init();
    info!("such information");
    warn!("o_O");
    error!("much error");
}
```

Then run your app with the environmental variable set:

```
RUST_LOG=trace cargo run
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

