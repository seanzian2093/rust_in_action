// A smart pointer type that reads from its pointer location without needing to copy it first
// Cow is short for copy on write
use std::borrow::Cow;
// Cstr is a C-like string type that allows Rust to read in zero-terminated strings
use std::ffi::CStr;
// c_char, a type-alias for Rust's i8 type, presents the possibility of a platform-specific nuances
use std::os::raw::c_char;
static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

pub fn main() {
    let a = 42;
    // String is a smart pointer type that holds a pointer to underlying array and a field to store its size
    let b: String;
    let c: Cow<str>;

    unsafe {
        // Can't directly be cast to *mut u8, so *const u8 first
        // *const T and *mut T is minimally different
        let b_ptr = &B as *const u8 as *mut u8;
        // from_raw_parts accepts a pointer that points to an array of bytes, a size and capacity parameter
        b = String::from_raw_parts(b_ptr, 10, 10);
        let c_ptr = &C as *const u8 as *const c_char;
        // from_ptr reads the pointer until it reaches the terminating 0, returns a Cow<str>
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }
    println!("a: {}, b: {}, c: {}", a, b, c);
}
