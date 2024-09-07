# Mesh

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![APACHE-2.0 licensed][apache-badge]][apache-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/mesh.svg
[crates-url]: https://crates.io/crates/mesh
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/roushou/mesh/blob/master/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/license-apache-blue.svg
[apache-url]: https://github.com/roushou/mesh/blob/master/LICENSE-APACHE
[actions-badge]: https://github.com/roushou/mesh/workflows/CI/badge.svg
[actions-url]: https://github.com/roushou/mesh/actions?query=workflow%3ACI+branch%3Amaster

Mesh is a Rust SDK designed to build AI-powered applications using (popular) LLM providers such as Anthropic, OpenAI, Replicate and more.

More information about this crate can be found in the [crate documentation](https://crates.io/crates/mesh).

## Getting started

Add `mesh` as a dependency in your application.

```sh
$ cargo add mesh
```

An example to create a message using Claude 3.5 Sonnet from Anthropic.

```rust
use anthropic_rs::{
    client::Client,
    completion::message::{Content, ContentType, Message, MessageRequest, Role},
    config::Config,
    models::claude::ClaudeModel,
};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY should be defined");

    let config = Config::new(api_key);
    let client = Client::new(config).unwrap();

    let message = MessageRequest {
        model: ClaudeModel::Claude35Sonnet,
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

    let result = client.create_message(message.clone()).await.unwrap();
    println!("{:?}", result);
}
```

## Licenses

This project is licensed under the [MIT license](./LICENSE-MIT) and [Apache-2.0](./LICENSE-APACHE) license.
