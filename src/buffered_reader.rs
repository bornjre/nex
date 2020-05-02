use std::io::Read;
use std::io;

use std::fs::File;



pub struct BufferedFileReader {
    file: File,
    size: u64,
    cursor: u64,
}

impl BufferedFileReader {
    pub fn new(filename: String) -> io::Result<Self> {
        let f = File::open(filename)?;
        let size = f.metadata()?.len();
        Ok(BufferedFileReader{file: f, size: size, cursor:0})
    }
    
    pub fn read(&mut self, buffer:&mut Vec<u8>) -> io::Result<u64>  {

        let mut read_len = buffer.len() as u64;

        if (self.size - self.cursor) < (buffer.len() as u64) {
            read_len = self.size - self.cursor;
        }
        let mut handle = self.file.by_ref().take(read_len);
        handle.read(buffer)?;
        self.cursor = self.cursor + read_len;

        Ok(read_len)
    }
}