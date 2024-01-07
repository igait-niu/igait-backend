use rand::random;

pub async fn run_inference(id: usize) -> f32 {
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    random::<f32>()
}