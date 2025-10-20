pub fn main () {
    // These two values have the same bit pattern but different types
    let a: u16 = 50115;
    let b: i16 = -15421;

    println!("a: {:016b} {}", a, a);
    println!("b: {:016b} {}", b, b);
}