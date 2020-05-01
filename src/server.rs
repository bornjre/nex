use tonic::{transport::Server, Request, Response, Status};

use nexproto::{ResultAck, ResultType, Work, WorkRequest};

use nexproto::work_provider_server::{WorkProvider, WorkProviderServer};

pub mod nexproto {
    tonic::include_proto!("nexproto");
}

#[derive(Debug, Default)]
struct WorkService {}

#[tonic::async_trait]
impl WorkProvider for WorkService {
    async fn need_work(&self, request: Request<WorkRequest>) -> Result<Response<Work>, Status> {
        println!("Got a request: {:?}", request);

        Ok(Response::new(Work { input: 9 }))
    }

    async fn upload_result(
        &self,
        request: Request<ResultType>,
    ) -> Result<Response<ResultAck>, Status> {
        println!("Got a request: {:?}", request);

        Ok(Response::new(ResultAck {
            status: 0,
            msg: "seems ok".to_string(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let workp = WorkService::default();

    Server::builder()
        .add_service(WorkProviderServer::new(workp))
        .serve(addr)
        .await?;

    Ok(())
}
