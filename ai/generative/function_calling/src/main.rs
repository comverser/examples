use ollama_rs::{
    coordinator::Coordinator,
    // generation::{chat::ChatMessage, tools::implementations::Calculator},
    generation::chat::ChatMessage,
    Ollama,
};

/// This function gets the weather of a city
///
/// * city - The city to get the weather of
#[ollama_rs::function]
async fn get_weather(city: String) -> Result<String, Box<dyn Error + Sync + Send>> {
    print!("[Inside get_weather]");
    Ok(
        reqwest::get(format!("https://wttr.in/{city}?format=%C+%t").as_str())
            .await?
            .text()
            .await?,
    )
}

#[tokio::main]
async fn main() {
    let ollama = Ollama::default();
    let history = vec![];
    // let tools = ollama_rs::tool_group![Calculator {}];
    let tools = ollama_rs::tool_group![get_weather];

    let model = String::from("llama3.2");
    let mut coordinator = Coordinator::new_with_tools(ollama, model, history, tools);

    // let user_message = "2+2*2=?".to_string();
    let user_message = "What's the weather in London?".to_string();
    let user_chat_message = ChatMessage::user(user_message);
    let response = coordinator.chat(vec![user_chat_message]).await.unwrap();

    println!(" Response: {:?}", response.message.content);
}
