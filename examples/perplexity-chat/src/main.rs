use perplexity::{
    client::{Client, CreateChatCompletion, Message, Model, Role},
    config::Config,
};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("PERPLEXITY_API_KEY")
        .expect("environment variable PERPLEXITY_API_KEY should be defined");

    let config = Config::new(api_key);
    let client = Client::new(config).unwrap();

    let messages: Vec<Message> = vec![Message {
        role: Role::User,
        content: "Find me the best pad thai restaurant in Bangkok".to_string(),
    }];
    let message = CreateChatCompletion::new(Model::Llama31SonarLargeOnline, messages);
    let result = client.create_completion(message).await.unwrap();
    println!("{:?}", result);
}
