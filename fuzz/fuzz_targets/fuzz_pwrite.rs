use honggfuzz::fuzz;

use scroll::Pwrite;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let mut write_into: Vec<u8> = vec![0; data.len()];
            write_into.pwrite(data, 0).unwrap();
        });
    }
}