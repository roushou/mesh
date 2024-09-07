# Anthropic Rust SDK

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![APACHE-2.0 licensed][apache-badge]][apache-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/anthropic-rs.svg
[crates-url]: https://crates.io/crates/anthropic-rs
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/roushou/mesh/blob/master/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/license-apache-blue.svg
[apache-url]: https://github.com/roushou/mesh/blob/master/LICENSE-APACHE
[actions-badge]: https://github.com/roushou/mesh/workflows/CI/badge.svg
[actions-url]: https://github.com/roushou/mesh/actions?query=workflow%3ACI+branch%3Amaster
[![Documentation](https://docs.rs/anthropic-rs/badge.svg)](https://docs.rs/anthropic-rs)

This is an unofficial Rust SDK for the Anthropic API.

## Installation

Add `anthropic-rs` as a dependency to your `Cargo.toml`

```sh
cargo add anthropic-rs
```

## Usage

An example to stream a message.

```rust,ignore
use anthropic_rs::{
    api::{
        message::{Content, ContentType, Message, MessageRequest, Role},
        stream::StreamEvent,
    },
    client::Client,
    config::Config,
    models::model::Model,
};
use futures_util::StreamExt;
use std::io::Write;

#[tokio::main]
async fn main() {
    let api_key = std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY should be defined");

    let config = Config::new(api_key);
    let client = Client::new(config).unwrap();

    let message = MessageRequest {
        model: Model::Claude35Sonnet,
        stream: Some(true),
        max_tokens: 1024,
        messages: vec![Message {
            role: Role::User,
            content: vec![Content {
                content_type: ContentType::Text,
                text: "Explain the theory of relativity".to_string(),
            }],
        }],
        ..Default::default()
    };

    let mut stream = client.stream_message(message.clone()).await.unwrap();

    while let Some(event) = stream.next().await {
        let event = event.unwrap();
        match event {
            StreamEvent::ContentBlockDelta(content) => {
                print!("{}", content.delta.text);
                std::io::stdout().flush().unwrap();
            }
            StreamEvent::MessageStop => break,
            _ => {}
        }
    }
}
```

## License

This project is licensed under the [MIT license](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE) license.
