
static GLOBAL: i32 = 1000;

// Create a local var within noop() so that something outside of main() has a mem address
fn noop() -> *const i32 {
    let noop_local = 12345;
    &noop_local as *const i32
}

// Create various values including values on the heap
pub fn main() {
    let local_str = "a";
    let local_int: i32 = 123;
    let boxed_str = Box::new('b');
    let boxed_int = Box::new(789);
    let fn_int = noop();
    
    println!("GLOBAL:        {:p}", &GLOBAL as *const i32);
    println!("local_str:     {:p}", local_str as *const str);
    println!("local_int:     {:p}", local_int as *const i32);
    // Box::into_raw consumes the Box and return a wrapped raw pointer
    println!("boxed_int:     {:p}", Box::into_raw(boxed_int));
    println!("boxed_str:     {:p}", Box::into_raw(boxed_str));
    println!("fn_int:        {:p}", fn_int);
    
}