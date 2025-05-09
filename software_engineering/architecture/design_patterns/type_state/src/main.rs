#![allow(unused)]

use std::collections::HashMap;
use std::marker::PhantomData;

pub struct PasswordManager<State> {
    master_pass: String,
    passwords: HashMap<String, String>,
    state: PhantomData<State>,
}

impl PasswordManager<Locked> {
    pub fn new(master_pass: String) -> Self {
        PasswordManager {
            master_pass,
            passwords: Default::default(),
            state: PhantomData,
        }
    }

    pub fn unlock(self, password: String) -> Result<PasswordManager<Unlocked>, &'static str> {
        if self.master_pass == password {
            Ok(PasswordManager {
                master_pass: self.master_pass,
                passwords: self.passwords,
                state: PhantomData,
            })
        } else {
            Err("Invalid master password")
        }
    }
}

impl PasswordManager<Unlocked> {
    pub fn lock(self) -> PasswordManager<Locked> {
        PasswordManager {
            master_pass: self.master_pass,
            passwords: self.passwords,
            state: PhantomData,
        }
    }

    pub fn add_password(&mut self, username: String, password: String) {
        self.passwords.insert(username, password);
    }

    pub fn list_passwords(&self) -> &HashMap<String, String> {
        &self.passwords
    }
}

struct Locked;
struct Unlocked;

fn main() {
    let manager = PasswordManager::new("admin_password".to_owned());
    let manager = manager.unlock("admin_password".to_owned()).unwrap();

    let mut manager = manager;
    manager.add_password("user".to_owned(), "user_password".to_owned());
    let result = manager.list_passwords();
    println!("{:?}", result);

    let manager = manager.lock();
}
