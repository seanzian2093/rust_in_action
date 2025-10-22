pub fn main() {
    // a lives on the stack
    let a: i32 = 40;
    // b lives on the heap
    let b: Box<i32> = Box::new(60);
    
    // To access 60, we need to dereference b
    println!("{} + {} = {}", a, b, a + *b);
}