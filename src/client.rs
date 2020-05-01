use nexproto::work_provider_client::WorkProviderClient;
use nexproto::{ResultType, WorkRequest};
use std::{thread, time};

pub mod nexproto {
    tonic::include_proto!("nexproto");
}

type Clientconn = WorkProviderClient<tonic::transport::channel::Channel>;

struct ClientContext {
    conn: Clientconn,
}

async fn run_client_loop() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut ctx : Option<ClientContext> = None;
    loop {
        loop {
            match connect().await {
                Ok(c) => {
                    ctx = Some( ClientContext{ conn: c,} );
                    break
                },
                Err(e) => println!(" {:?} cannot connect sleeping before retrying", e),
            }
            thread::sleep(time::Duration::from_secs(5));
        }
        loop {
            let mut c = match ctx {
                Some(c) => c,
                None => break,
            };

            c  = do_work(c).await?;
            ctx = Some(c);
            thread::sleep(time::Duration::from_secs(5));
        }
    }
}


async fn connect<'a>() -> Result<Clientconn, Box<dyn std::error::Error>> {
    let c: Clientconn = WorkProviderClient::connect("http://[::1]:50051").await?;
    Ok(c)
}


async fn do_work(mut ctx: ClientContext) -> Result<ClientContext, Box<dyn std::error::Error>> {
    let request = tonic::Request::new(WorkRequest {
        r#type: "any".to_string(),
    });

    let mut response = ctx.conn.need_work(request).await?;

    let response = response.get_mut();

    println!("do work {}", response.input);

    let output = response.input * response.input;

    let request = tonic::Request::new(ResultType {
        status: 0,
        output: output,
    });

    let response = ctx.conn.upload_result(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(ctx)
}

#[tokio::main]
async fn main() {
    run_client_loop().await;
}
