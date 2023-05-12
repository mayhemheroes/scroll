use honggfuzz::fuzz;

use scroll::Pread;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let _str: &str = data.pread(0).unwrap();
        });
    }
}