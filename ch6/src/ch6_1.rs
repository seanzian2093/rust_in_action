pub fn main() {
    static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
    static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];
    let a = 42;
    let b = &B;
    let c = &C;
    // {:p} syntax asks Rust to format a variable as pointer and print the mem address that is pointed to
    println!("a: {}, b: {:p}, c: {:p}", a, b, c);
}
