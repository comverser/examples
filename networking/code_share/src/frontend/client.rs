mod template;

use axum::{response::IntoResponse, routing::get, Router};
use template::{HtmlTemplate, IndexTemplate};

#[tokio::main]
async fn main() {
    let app: Router = Router::new().route("/", get(index_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index_handler() -> impl IntoResponse {
    HtmlTemplate(IndexTemplate {})
}
