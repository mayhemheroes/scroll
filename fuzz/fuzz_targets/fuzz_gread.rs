use honggfuzz::fuzz;

use scroll::Pread;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let _str: &str = data.gread(&mut 0).unwrap();
        });
    }
}