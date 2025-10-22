pub fn main() {
    let a: i64 = 42;
    // Cast &a to a constant raw pointer i64
    // *mut T and *const T always point to starting byte of T
    // i64 is of 64 bits, i.e., 8 bytes
    let a_ptr = &a as *const i64;
    println!("a: {} ({:p})",a,a_ptr);
}