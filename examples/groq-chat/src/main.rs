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
