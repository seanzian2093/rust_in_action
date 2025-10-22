// Computing platforms differ as to how numbers are read.
// Some read 4-bytes of an i32 from left to right, some right to left.
// This poses a problem when our program is designed to be written by one computer and loaded by another.
// `byteorder` crate comes to the rescue. This is a toy app for demo.

// cargo run -p ch7/ch7-write123
// cargo run --bin ch7-write123

// As files support the ability to seek(), moving forward and backward to different positions.
// We need to use `Cursor` to enable a Vec<T> to mocking being a file

use std::io::Cursor;
// `LittleEndian` is a type argument for various read() and write()
use byteorder::{ WriteBytesExt, ReadBytesExt, LittleEndian};

fn write_numbers_to_file() -> (u32, i8, f64) {
    let mut w = vec![];
    let one: u32 = 1;
    let two: i8 = 2;
    let three: f64 = 3.0;

    w.write_u32::<LittleEndian>(one).unwrap();
    println!("{:?}", &w);
    // Single byte types i8 and u8 don't take an endianness parameter
    w.write_i8(two).unwrap();
    println!("{:?}", &w);
    w.write_f64::<LittleEndian>(three).unwrap();
    println!("{:?}", &w);

    (one, two, three)
}

fn read_numbers_from_file() -> (u32, i8, f64) {
    let mut r = Cursor::new(vec![1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 8, 64]);
    let one = r.read_u32::<LittleEndian>().unwrap();
    let two = r.read_i8().unwrap();
    let three = r.read_f64::<LittleEndian>().unwrap();

    (one, two, three)
}

fn main() {
    let (one, two, three) = write_numbers_to_file();
    let (one_, two_, three_) = read_numbers_from_file();

    assert_eq!(one, one_);
    assert_eq!(two, two_);
    assert_eq!(three, three_);
}