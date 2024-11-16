use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize)]
pub struct ProductData {
    pub name: String,
}
