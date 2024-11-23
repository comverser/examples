use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Query, WebSocketUpgrade,
    },
    response::IntoResponse,
    Extension,
};
use tokio::sync::{mpsc, Mutex};

use crate::{
    cache::Cache,
    model::{ChatMessage, Client, QueryParams, Response},
};

pub async fn ws_handler(
    Extension(cache): Extension<Arc<Cache>>,
    ws: WebSocketUpgrade,
    Query(query_params): Query<QueryParams>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, cache, query_params))
}

async fn handle_socket(socket: WebSocket, cache: Arc<Cache>, query_params: QueryParams) {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let socket = Arc::new(Mutex::new(socket));

    cache.add_client(
        query_params.room_id.clone(),
        Client {
            user_id: query_params.user_id.clone(),
            sender: tx,
        },
    );

    loop {
        let mut socket = socket.lock().await;
        tokio::select! {
            Some(msg) = rx.recv() => {
                println!("Sending message to client");

                if socket.send(msg).await.is_err() {
                    println!("Failed to send message, closing connection");
                    break;
                }
            }
            result = socket.recv() => {
                match result {
                    Some(Ok(msg)) => {
                        match msg {
                            Message::Text(txt) => {
                                println!("Received text message: {}", txt);
                                if let Ok(new_msg) = serde_json::from_str::<ChatMessage>(&txt) {
                                    cache.add_data(query_params.room_id.clone(), new_msg.code.clone());
                                    if let Some(clients) = cache.get_clients(&query_params.room_id) {
                                        for client in clients {
                                            println!("Broadcasting message to client {}", client.user_id);
                                            let response = Response {
                                                room_id: query_params.room_id.clone(),
                                                code: new_msg.code.clone(),
                                            };

                                            let _ = client.sender.send(Message::Text(serde_json::to_string(&response).unwrap()));
                                        }
                                    }
                                    else {
                                        println!("No clients in room: {}", query_params.room_id);
                                    }
                                } else {
                                    println!("Failed to parse message: {}", txt);
                                }
                            }
                            Message::Close(_) => {
                                println!("Client disconnected");
                                break;
                            }
                            _ => {}
                        }
                    }
                    _ => {break;}
                }
            }
        }
    }
}
