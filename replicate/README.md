# Replicate Rust SDK

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![APACHE-2.0 licensed][apache-badge]][apache-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/replic.svg
[crates-url]: https://crates.io/crates/replic
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/roushou/mesh/blob/master/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/license-apache-blue.svg
[apache-url]: https://github.com/roushou/mesh/blob/master/LICENSE-APACHE
[actions-badge]: https://github.com/roushou/mesh/workflows/CI/badge.svg
[actions-url]: https://github.com/roushou/mesh/actions?query=workflow%3ACI+branch%3Amaster

This is an unofficial Rust SDK for the Anthropic API.

## Installation

Add `replic` as a dependency to your `Cargo.toml`

```sh
$ cargo add replic
```

## Usage

An example to get collections

```rust
use replic::{client::Client, config::Config};

#[tokio::main]
async fn main() {
    let config = Config::from_env().unwrap();
    let client = Client::new(config).unwrap();

    let collections = client.collections().await.unwrap();
    println!("{:?}", collections);
}
```

## License

This project is licensed under the [MIT license](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE) license.
