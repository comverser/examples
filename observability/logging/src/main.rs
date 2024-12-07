use std::env;

use log::{debug, error, info, warn};

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // let username = "Alice";
    // let password = "mypassword";
    // let username = "admin";
    // let password = "admin";
    let username = "";
    let password = "";

    debug!("Checking Credentials..");

    if username.is_empty() || password.is_empty() {
        error!("Credentials are empty");
        return;
    }

    if username == "admin" && password == "admin" {
        info!("Its admin user");
    } else {
        warn!("Username or password is incorrect");
    }
}
