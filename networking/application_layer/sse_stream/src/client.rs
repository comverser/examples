use futures::TryStreamExt;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let response = Client::new()
        .get("http://localhost:3000/sse")
        .send()
        .await
        .unwrap();

    if response.status().is_success() {
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.try_next().await.unwrap() {
            let chunk_data = std::str::from_utf8(&chunk).unwrap();
            println!("{:?}", chunk_data);
        }
    } else {
        println!("Failed to connect to server");
    }
}
