use replic::{client::Client, config::Config};

#[tokio::main]
async fn main() {
    let config = Config::from_env().unwrap();
    let client = Client::new(config).unwrap();

    let collections = client.collections().await.unwrap();
    println!("{:?}", collections);
}
