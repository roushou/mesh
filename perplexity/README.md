# Perplexity Rust SDK

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![APACHE-2.0 licensed][apache-badge]][apache-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/perplexity.svg
[crates-url]: https://crates.io/crates/perplexity
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/roushou/mesh/blob/master/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/license-apache-blue.svg
[apache-url]: https://github.com/roushou/mesh/blob/master/LICENSE-APACHE
[actions-badge]: https://github.com/roushou/mesh/workflows/CI/badge.svg
[actions-url]: https://github.com/roushou/mesh/actions?query=workflow%3ACI+branch%3Amaster

This is an unofficial Rust SDK for the Perplexity API.

More information about this crate can be found in the [crate documentation](https://crates.io/crates/perplexity).

## Installation

Add `perplexity` as a dependency to your `Cargo.toml`

```sh
$ cargo add perplexity
```

## Usage

An example to create a completion.

```rust,ignore
use perplexity::{
    client::{Client, CreateChatCompletion, Model},
    config::Config,
};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("PERPLEXITY_API_KEY")
        .expect("environment variable PERPLEXITY_API_KEY should be defined");

    let config = Config::new(api_key);
    let client = Client::new(config).unwrap();

    let message = CreateChatCompletion::new(Model::Llama31SonarLargeOnline);
    let result = client.create_completion(message.clone()).await.unwrap();
    println!("{:?}", result);
}
```

## License

This project is licensed under the [MIT license](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE) license.
