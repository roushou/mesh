use replic::{
    client::{Client, CreatePrediction},
    config::Config,
};

#[tokio::main]
async fn main() {
    let config = Config::from_env().unwrap();
    let client = Client::new(config).unwrap();

    let collections = client
        .create_prediction(CreatePrediction {
            version: "f2ab8a5bfe79f02f0789a146cf5e73d2a4ff2684a98c2b303d1e1ff3814271db".to_string(),
            input: serde_json::json!({
                "prompt": "black forest gateau cake spelling out the words \"FLUX SCHNELL\", tasty, food photography, dynamic shot"
            }),
            webhook: None,
            webhook_event_filters: None,
        })
        .await
        .unwrap();
    println!("{:?}", collections);
}
