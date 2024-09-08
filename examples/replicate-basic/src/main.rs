use replic::{
    client::{Client, CreateModelPrediction},
    config::Config,
};

#[tokio::main]
async fn main() {
    let config = Config::from_env().unwrap();
    let client = Client::new(config).unwrap();

    let payload = CreateModelPrediction {
        owner: "black-forest-labs".to_string(),
        name: "flux-schnell".to_string(),
        input: serde_json::json!({
            "prompt": "3D model of a baby dragon",
            "num_outputs": 1,
            "aspect_ratio": "1:1",
            "output_format": "webp",
            "output_quality": 100
        }),
    };
    let prediction = client.create_model_prediction(payload).await.unwrap();
    println!("{:?}", prediction);
}
