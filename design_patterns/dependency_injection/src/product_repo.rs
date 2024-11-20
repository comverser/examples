use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use uuid::Uuid;

use crate::model::Product;

pub trait ProductRepo: Send + Sync {
    fn get_product(&self, id: Uuid) -> Option<Product>;

    fn save_product(&self, product: &Product);
}

// data structure
#[derive(Debug, Clone, Default)]
pub struct InMemoryProductRepo {
    pub map: Arc<Mutex<HashMap<Uuid, Product>>>,
}

impl ProductRepo for InMemoryProductRepo {
    fn get_product(&self, id: Uuid) -> Option<Product> {
        let map = self.map.lock().unwrap();
        map.get(&id).cloned()
    }

    fn save_product(&self, product: &Product) {
        let mut map = self.map.lock().unwrap();
        map.insert(product.id, product.clone());
    }
}
