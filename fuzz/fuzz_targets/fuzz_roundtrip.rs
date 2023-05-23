use honggfuzz::fuzz;

use std::io::Cursor;
use scroll::{Pread, Pwrite, IOread};

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let mut bytes = Cursor::new(data);
            let mut buffer: Vec<u8> = vec![0; data.len()*2];

            for i in 0..data.len() {
                let byte = bytes.ioread::<u8>().unwrap();
                buffer.pwrite(byte, i*2).unwrap();
                buffer.pwrite('\0' as u8, i*2 + 1).unwrap();
            }

            for i in 0..data.len() {
                let _ = buffer.pread::<u8>(data.len() - i - 1).unwrap();
            }
        });
    }
}