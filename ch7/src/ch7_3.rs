use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;

// Use r## to build raw string literal to allow multiline
const INPUT: &'static [u8] = br#"
fn main() {
    println!("Hello, world!);
"#;

pub fn main() -> std::io::Result<()> {
    let mut buffer: Vec<u8> = vec!();
    INPUT.read_to_end(&mut buffer)?;

    let mut position_in_put = 0;
    for line in buffer.chunks(BYTES_PER_LINE) {
        println!("0x{:08x} ", position_in_put);
        for byte in line {
            print!("{:02x} ", byte);
        }
        println!();
        position_in_put += BYTES_PER_LINE;
    }

    Ok(())
}