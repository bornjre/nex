use nexproto::work_provider_client::WorkProviderClient;
use nexproto::{ResultType, WorkRequest, BinaryRequest};
use std::{thread, time};
use std::fs::File;
use wasmer_runtime::{error, imports, instantiate, Func};
use std::io::Read;

pub mod nexproto {
    tonic::include_proto!("nexproto");
}

mod file_utilities;
use file_utilities::wasm_file;

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

            
            let work = get_work(&mut c).await?;
            let mut file = file_get(&mut c, work.1).await?;

            let result = do_work(&mut c, &mut file, work).await?;
            upload_result(&mut c, result).await?;

            ctx = Some(c);
            thread::sleep(time::Duration::from_secs(5));
        }
    }
}

async fn file_get( ctx:&mut ClientContext, binary_id: i64) -> Result<File, Box<dyn std::error::Error>> {
    let file = wasm_file(binary_id);
    let f =  match File::open(file.clone()) {
        Ok(f) => f,
        Err(_) => {
            get_binary(ctx, binary_id).await?;
            File::open(file)?
        },
    };
    Ok(f)
}



async fn connect<'a>() -> Result<Clientconn, Box<dyn std::error::Error>> {
    let c: Clientconn = WorkProviderClient::connect("http://[::1]:50051").await?;
    Ok(c)
}

struct Work(i64, i64);

async fn get_work( ctx:&mut ClientContext) -> Result<Work, Box<dyn std::error::Error>> {
    let request = tonic::Request::new(WorkRequest {
        r#type: "any".to_string(),
    });

    let mut response = ctx.conn.need_work(request).await?;

    let response = response.get_mut();

    println!("do work {:?}", response);
    Ok(Work(response.input, response.binary_id))
}

async fn get_binary( ctx:&mut ClientContext, binary_id: i64) -> Result<(), Box<dyn std::error::Error>> {
    println!("Downloading binary");

    let binary_response = ctx.conn.get_binary(
        tonic::Request::new(BinaryRequest {
            id: binary_id,
        })
    ).await?;
    let mut stream = binary_response.into_inner();
    let mut file = File::create(wasm_file(binary_id))?;
    


    while let Some(message) = stream.message().await? {

        std::io::Write::write_all(&mut file, &message.data.to_vec())?;

        println!("Messeage: {:?}, {:?}", message.status, message.description);
    }
    println!("Finished downloading binary");
    Ok(())
}

async fn do_work( ctx:&mut ClientContext, f:&mut File, w: Work) -> Result<i64, Box<dyn std::error::Error>> {

    let mut buf = Vec::new();

    f.read_to_end(&mut buf)?;

    let import_object = imports! {};

    let instance = instantiate(&buf, &import_object)?;

    let add_one: Func<u32, u32> = instance.func("add_one")?;

    let result = add_one.call(w.0 as u32)?;

    // Log the new value
    println!("Result: {}", result);



    Ok(result as i64)
}

async fn upload_result( ctx:&mut ClientContext, output: i64) -> Result<(), Box<dyn std::error::Error>> {

    let request = tonic::Request::new(ResultType {
        status: 0,
        output: output,
    });

    let response = ctx.conn.upload_result(request).await?;

    println!("Response={:?}", response);

    Ok(())
}


#[tokio::main]
async fn main() {
    run_client_loop().await;
}
