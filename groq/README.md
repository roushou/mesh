# Groq Unofficial Rust SDK

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![APACHE-2.0 licensed][apache-badge]][apache-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/gruq.svg
[crates-url]: https://crates.io/crates/gruq
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/roushou/mesh/blob/master/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/license-apache-blue.svg
[apache-url]: https://github.com/roushou/mesh/blob/master/LICENSE-APACHE
[actions-badge]: https://github.com/roushou/mesh/workflows/CI/badge.svg
[actions-url]: https://github.com/roushou/mesh/actions?query=workflow%3ACI+branch%3Amaster

This is an unofficial Rust SDK for the [Groq API](https://console.groq.com/docs/quickstart).

More information about this crate can be found in the [crate documentation](https://crates.io/crates/gruq).

## Installation

Add `gruq` as a dependency to your `Cargo.toml`

```sh
$ cargo add gruq
```

## Usage

An example to create a completion.

```rust,ignore
use gruq::{
    chat::message::{CreateChatCompletion, Message, Role},
    client::Client,
    config::Config,
    models::Model,
};

#[tokio::main]
async fn main() {
    let config = Config::from_env().unwrap();
    let client = Client::new(config).unwrap();

    let messages: Vec<Message> = vec![Message {
        role: Role::User,
        content: "Hello World".to_string(),
        name: None,
    }];
    let completion_request = CreateChatCompletion::new(Model::Llama38B, messages);
    let completion = client
        .chat
        .create_completion(completion_request)
        .await
        .unwrap();
    println!("{:?}", completion);
}
```

## License

This project is licensed under the [MIT license](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE) license.
