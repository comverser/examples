use axum::{http::StatusCode, routing::post, Router};
use tokio::net::TcpListener;
use validator::ValidatedPayload;

mod model;
mod validator;

async fn handler(ValidatedPayload(data): ValidatedPayload<model::Employee>) -> StatusCode {
    println!("{:?}", data);
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    let app: Router = Router::new().route("/", post(handler));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("{:?}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
