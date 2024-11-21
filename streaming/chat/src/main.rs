use futures::TryStreamExt;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct ChatContent {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    delta: ChatContent,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatChoice>,
}

#[tokio::main]
async fn main() {
    let body = json!({"model":"gpt-3.5-turbo", "messages": [{"role": "user", "content":"Write a essay of 20 words on tech."}], "stream": true});

    let client = reqwest::Client::new();

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .body(body.to_string())
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer API_KEY")
        .send()
        .await
        .unwrap();

    if response.status() == StatusCode::OK {
        let mut stream = response.bytes_stream();

        while let Some(chunk) = stream.try_next().await.unwrap() {
            let chunk_string = std::str::from_utf8(&chunk).unwrap();

            let split_data = chunk_string.split("data:");

            for data in split_data {
                let data_chunk = data.trim();

                if data_chunk.is_empty() {
                    continue;
                }

                if data_chunk == "[DONE]" {
                    break;
                }

                if let Ok(value) = serde_json::from_str::<ChatCompletionResponse>(data_chunk) {
                    match value.choices.first() {
                        None => {}
                        Some(content) => {
                            println!("{:?}", content);
                        }
                    }
                }
            }
        }
    } else {
        println!("Error: {:?}", response.text().await.unwrap());
    }
}
