use opai::{
    chats::message::{CreateChatCompletion, Message, Role},
    client::Client,
    config::Config,
    models::gpt::GptModel,
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
    let request = CreateChatCompletion::new(GptModel::GPT4o, messages);
    let completion = client.chat.create_completion(request).await.unwrap();
    println!("{:?}", completion);
}
