use std::{convert::Infallible, time::Duration};

use axum::{
    response::{sse::Event, Sse},
    routing::get,
    Router,
};
use chrono::Utc;
use futures::{stream, Stream};
use tokio_stream::StreamExt;

fn get_data() -> String {
    format!(
        "New data from server at: {:?}",
        Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
    )
}

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // Create a stream that repeatedly generates new SSE events with data from get_data()
    let stream = stream::repeat_with(|| Event::default().data(get_data()))
        // Wrap each event in a Result
        .map(Ok)
        // Throttle the stream to emit an event every 5 seconds
        .throttle(Duration::from_secs(3));

    // Create an SSE response with the stream
    Sse::new(stream)
        // Enable keep-alive to send a ping every 5 seconds to keep the connection alive
        .keep_alive(axum::response::sse::KeepAlive::new().interval(Duration::from_secs(5)))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/sse", get(sse_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
