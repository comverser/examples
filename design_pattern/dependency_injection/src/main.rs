use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use model::{Product, ProductData};
use product_repo::{InMemoryProductRepo, ProductRepo};
use tokio::net::TcpListener;
use uuid::Uuid;

mod model;
mod product_repo;

#[derive(Clone)]
struct AppState<T> {
    product_repo: T,
}

async fn create_product<T>(
    State(state): State<AppState<T>>,
    Json(data): Json<ProductData>,
) -> Json<Product>
where
    T: ProductRepo,
{
    let product = Product {
        id: uuid::Uuid::new_v4(),
        name: data.name,
    };

    state.product_repo.save_product(&product);

    Json(product)
}

async fn get_product<T>(
    State(state): State<AppState<T>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Product>, StatusCode>
where
    T: ProductRepo,
{
    match state.product_repo.get_product(id) {
        Some(product) => Ok(Json(product)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[tokio::main]
async fn main() {
    let product_repo = InMemoryProductRepo::default();

    let app: Router = Router::new()
        .route("/products/:id", get(get_product::<InMemoryProductRepo>))
        .route("/products", post(create_product::<InMemoryProductRepo>))
        .with_state(AppState { product_repo });

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on {:?}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
