use hello_world::{
    greeter_server::{Greeter, GreeterServer},
    HelloRequest, HelloResponse,
};
use tonic::{transport::Server, Request, Response, Status};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default, Debug)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloResponse {
            message: format!("Hello {} from Server Side", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:50051".parse()?;
    let greeter_service = MyGreeter::default();
    println!("GreeterServer listening on {}", address);

    Server::builder()
        .add_service(GreeterServer::new(greeter_service))
        .serve(address)
        .await?;

    Ok(())
}
