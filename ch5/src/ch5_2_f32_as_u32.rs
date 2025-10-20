pub fn main() {
    let a: f32 = 42.42;
    // Take bits of `42.42_f32` and interpret it as u32, i.e., decimal integer
    let franken_type: u32 = unsafe { std::mem::transmute(a) };


    // Display that integer
    println!("\nfranken_type: {}", franken_type);
    // format `franken_type` as a binary with at most 32 zeroes padded on the left
    println!("franken_type: {:032b}", franken_type);

    // Take the bits of `42.42_f32` and interpret it as `f32`, i.e., `42.42_f32`
    let b: f32 = unsafe { std::mem::transmute(franken_type) };
    println!("b: {}", b);
    assert_eq!(a, b);
}