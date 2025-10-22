// This program crashes at i=0 because ptr=0x0 which is a NULL pointer
// so *ptr is dereferencing a NULL pointer
pub fn main() {
    let mut n_nonzero = 0;

    for i in 0..10000 {
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