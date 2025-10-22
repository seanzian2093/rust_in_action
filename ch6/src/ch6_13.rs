// This program still crashes though we avoided NULL pointer exception
// Due to segmentation fault, i.e., attempting to access memory region that is not entitled to
pub fn main() {
    let mut n_nonzero = 0;

    // Start from 1 to avoid NULL pointer exception
    for i in 1..10000 {
        // Convert i to a *const T that is a raw pointer that points to a mem address
        let ptr = i as *const u8;
        println!("{} -> {:p}", i, ptr);
        // Better to use unsafe to read the value at an address pointed by a raw pointer
        let byte_at_addr = unsafe { *ptr };

        if byte_at_addr != 0 {
            n_nonzero += 1;
        }
    }

    println!("non-zero bytes in memory: {}", n_nonzero);
}