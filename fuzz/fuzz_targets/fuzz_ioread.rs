use honggfuzz::fuzz;

use std::io::Cursor;
use scroll::IOread;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let mut bytes = Cursor::new(data);

            for _i in 0..data.len() {
                let _b = bytes.ioread::<u8>().unwrap();
            }
        });
    }
}