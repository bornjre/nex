use tonic::{transport::Server, Request, Response, Status};
use tokio::sync::mpsc;

use nexproto::{ResultAck, ResultType, WorkResponse, BinaryResponse, WorkRequest, BinaryRequest};
use nexproto::work_provider_server::{WorkProvider, WorkProviderServer};
use lazy_static::lazy_static;
use std::collections::HashMap;


mod file_utilities;
use file_utilities::BufferedFileReader;

pub mod nexproto {
    tonic::include_proto!("nexproto");
}


lazy_static! {
    static ref GLOBAL_FILE_INDEX: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "contrib/wasm_files/add.wasm");
        m
    };
}

#[derive(Debug, Default)]
struct WorkService {}

#[tonic::async_trait]
impl WorkProvider for WorkService {
    
    async fn need_work(&self, request: Request<WorkRequest>) -> Result<Response<WorkResponse>, Status> {
        println!("Got a request: {:?}", request);

        Ok(Response::new(WorkResponse { input: 9, binary_id: 0 }))
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

    type GetBinaryStream = mpsc::Receiver<Result<BinaryResponse, Status>>;
    async fn get_binary(&self, request: Request<BinaryRequest>,) -> Result<Response<Self::GetBinaryStream>, Status>  {
        println!("Got a request: {:?}", request);

        let (mut tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            let req =  request.get_ref();

            println!("Client wants to get file: {:?}",req.id  );

            let file = GLOBAL_FILE_INDEX.get(&(( req.id as i32) as u32)).unwrap().to_string();
            dbg!(&file);
            let mut reader = BufferedFileReader::new(file).unwrap();
            let mut buffer = vec![0; 1024 as usize];
            
            loop {
                let mut status = 0;
                let read = reader.read(&mut buffer).unwrap();
                if read < buffer.len() as u64 {
                    status = 1;
                    break;
                }
                tx.send(Ok(BinaryResponse{ status: status, data: buffer[1..(read as usize)].to_vec(), description: "".to_string(),})).await.unwrap();
            }
        });

        Ok(Response::new(rx))
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
