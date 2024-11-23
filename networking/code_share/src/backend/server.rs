use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, routing::get, Extension, Router};
use cache::Cache;
use socket::ws_handler;
use tower_http::cors::{Any, CorsLayer};

mod cache;
mod model;
mod socket;

#[tokio::main]
async fn main() {
    let cache = Cache::new();
    let app = Router::new()
        .route("/send", get(ws_handler))
        .route("/history/:id", get(history_handler))
        .layer(Extension(Arc::new(cache)))
        .layer(CorsLayer::new().allow_origin(Any));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn history_handler(
    Path(id): Path<String>,
    Extension(cache): Extension<Arc<Cache>>,
) -> Result<String, StatusCode> {
    match cache.get_data(&id) {
        Some(code) => Ok(code),
        None => Err(StatusCode::NOT_FOUND),
    }
}
