# OpenAI Rust SDK

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![APACHE-2.0 licensed][apache-badge]][apache-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/opai.svg
[crates-url]: https://crates.io/crates/opai
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/roushou/mesh/blob/master/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/license-apache-blue.svg
[apache-url]: https://github.com/roushou/mesh/blob/master/LICENSE-APACHE
[actions-badge]: https://github.com/roushou/mesh/workflows/CI/badge.svg
[actions-url]: https://github.com/roushou/mesh/actions?query=workflow%3ACI+branch%3Amaster

This is an unofficial Rust SDK for the OpenAI API.

More information about this crate can be found in the [crate documentation](https://crates.io/crates/opai).

## Installation

Add `opai` as a dependency to your `Cargo.toml`

```sh
$ cargo add opai
```

## Usage

An example to create a completion.

```rust,ignore
use opai::{
    chats::message::{CreateChatCompletion, Message, Role},
    client::Client,
    config::Config,
    models::{gpt::Gpt, Model},
};

#[tokio::main]
async fn main() {
    let config = Config::from_env().unwrap();
    let client = Client::new(config).unwrap();

    let messages: Vec<Message> = vec![Message {
        content: "Hello World".into(),
        role: Role::User,
        name: None,
    }];
    let request = CreateChatCompletion::new(Model::Gpt(Gpt::GPT4), messages);
    let completion = client.chat.create_completion(request).await.unwrap();
    println!("{:?}", completion);
}
```

## License

This project is licensed under the [MIT license](../LICENSE-MIT) and [Apache-2.0](../LICENSE-APACHE) license.
