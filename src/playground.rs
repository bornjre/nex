
use std::io::Read;
use std::io;

use std::fs::File;
use std::str;

const BUF_SIZE:u64 = 7;

mod buffered_reader;

use buffered_reader::BufferedFileReader;

fn simple_read() -> io::Result<()> {
    
    let mut f = File::open("contrib/test.txt").unwrap();
    let mut buffer = vec![0; BUF_SIZE as usize];
    let size = f.metadata().unwrap().len();
    
    println!("** {:?}", size);
    
    for x in (0..size).step_by(BUF_SIZE as usize) {
    
        println!("** {:?}", x);
        let mut handle = f.by_ref().take(BUF_SIZE);
        
        if (size - x) < BUF_SIZE {
            let left = size - x;
            let mut handle = f.by_ref().take(size-x);
            handle.read(&mut buffer)?;
            println!("** {:?}", str::from_utf8(&buffer[1..(left as usize)]));
            continue;
        }
        
        handle.read(&mut buffer)?;
        println!("** {:?}", str::from_utf8(&buffer));
    }
    
    Ok(())
}

fn better_read() -> io::Result<()> {

    let mut reader = BufferedFileReader::new("contrib/test.txt".to_string())?;
    let mut buffer = vec![0; BUF_SIZE as usize];
       
    loop {
        let read = reader.read(&mut buffer)?;
        println!("READ: {:?}", str::from_utf8(&buffer[1..(read as usize)]));
        if read < buffer.len() as u64 {
            break;
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    better_read()
}