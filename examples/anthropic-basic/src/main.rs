use anthropic_rs::{
    client::Client,
    completion::message::{Content, ContentType, Message, MessageRequest, Role},
    config::Config,
    models::claude::ClaudeModel,
};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .expect("environment variable ANTHROPIC_API_KEY should be defined");

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
