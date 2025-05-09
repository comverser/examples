use jsonrpc_core::{Error, IoHandler, Params};
use jsonrpc_http_server::ServerBuilder;
use serde_json::Value;

fn main() {
    let mut io = IoHandler::default();

    io.add_method("say_hello", |_params| async {
        Ok(Value::String("hello".to_string()))
    });

    io.add_method("add", |params: Params| async {
        let tuple = params.parse::<(u32, u32)>();
        match tuple {
            Ok((a, b)) => Ok(Value::Number(serde_json::Number::from(a + b))),
            Err(ex) => {
                eprintln!("Error: {:?}", ex);
                Err(Error::invalid_params("Expected two integer parameters"))
            }
        }
    });

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"0.0.0.0:3000".parse().unwrap())
        .unwrap();

    println!("Server running on");
    server.wait();
}
