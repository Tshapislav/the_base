//!ba2

use std::io::Read;


fn main() {
    let mut input_bytes: Vec<u8> = Vec::new();
    let bytes = std::io::stdin().read_to_end(&mut input_bytes).unwrap();

    let input = std::str::from_utf8(&input_bytes).unwrap();

    let lines = input.matches("\n").count();
    let words = input.split_whitespace().count();

    println!("{} {} {}", lines, words, bytes);
}
