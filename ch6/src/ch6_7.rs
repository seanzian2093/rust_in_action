use std::mem::drop;

pub fn main() {
    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);
    
    let result1 = *a + *b + *c;
    
    // Use drop to free memory for other use
    drop(a);
    let d = Box::new(1);
    let result2 = *b + *d + *c;
    
    println!("{} {}", result1, result2);
    
}