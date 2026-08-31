//! ba01
//! Read input and print count of bytes.

use std::io::Read;

fn main() {
    let mut input_bytes: Vec<u8> = Vec::new();
    match std::io::stdin().read_to_end(&mut input_bytes) {
        Ok(bytes) => println!("{}", bytes),
        Err(_) => println!("ERROR")
    };
}
