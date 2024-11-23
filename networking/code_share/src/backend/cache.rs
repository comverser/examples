use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::model::Client;

#[derive(Clone)]
pub struct Cache {
    client_cache: Arc<Mutex<HashMap<String, Vec<Client>>>>,
    data_cache: Arc<Mutex<HashMap<String, String>>>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            client_cache: Arc::new(Mutex::new(HashMap::new())),
            data_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_data(&self, room_id: String, data: String) {
        let mut cache = self.data_cache.lock().unwrap();
        cache.insert(room_id, data);
    }

    pub fn get_data(&self, room_id: &String) -> Option<String> {
        let cache = self.data_cache.lock().unwrap();
        cache.get(room_id).map(|data| data.clone())
    }

    pub fn add_client(&self, room_id: String, client: Client) {
        let mut rooms = self.client_cache.lock().unwrap();
        rooms.entry(room_id).or_insert_with(Vec::new).push(client);
    }

    pub fn get_clients(&self, room_id: &String) -> Option<Vec<Client>> {
        let rooms = self.client_cache.lock().unwrap();
        rooms.get(room_id).cloned()
    }
}
