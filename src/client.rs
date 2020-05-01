
use nexproto::work_provider_client::{WorkProviderClient};
use nexproto::{ WorkRequest, ResultType};

pub mod nexproto {
    tonic::include_proto!("nexproto");
}

async fn do_work() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = WorkProviderClient::connect("http://[::1]:50051").await?;
    
    let mut counter:i32 = 0;

    loop {
        counter = counter + 1;
        if counter > 3 {
            break;
        }

        let request = tonic::Request::new(WorkRequest{
            r#type: "any".to_string(),
        });
    
        let mut response = client.need_work(request).await?;
    
        let response = response.get_mut();
        
        println!("do work {}", response.input);

        let output = response.input * response.input;

        let request = tonic::Request::new(ResultType{
            status: 0,
            output:output,
        });
    
        let mut response = client.upload_result(request).await?;
        println!("RESPONSE={:?}", response);

    }


    Ok(())
}

#[tokio::main]
async fn main() {
    do_work().await;
}