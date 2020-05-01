use tonic::{transport::Server, Request, Response, Status};

use nexproto::greeter_server::{Greeter, GreeterServer};
use nexproto::{
    HelloReply,
    HelloRequest,
    WorkRequest,
    Work,
    ResultType,
    ResultAck
 };

use nexproto::work_provider_server::{WorkProvider, WorkProviderServer };


pub mod nexproto {
    tonic::include_proto!("nexproto");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = nexproto::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[derive(Debug, Default)]
struct WorkService {}

#[tonic::async_trait]
impl WorkProvider for WorkService {
    async fn need_work( &self, request: Request<WorkRequest>) -> Result<Response<Work>, Status> {

        println!("Got a request: {:?}", request);

        Ok(Response::new(Work{input: 9}))
    }

    async fn upload_result( &self,request: Request<ResultType>) -> Result<Response<ResultAck>, Status> {

        println!("Got a request: {:?}", request);

        Ok(Response::new(ResultAck{status:0, msg:"seems ok".to_string() }))
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();
    let workp = WorkService::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .add_service(WorkProviderServer::new(workp))
        .serve(addr)
        .await?;

    Ok(())
}