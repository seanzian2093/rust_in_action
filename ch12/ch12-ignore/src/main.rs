use libc::{SIGTERM, SIG_DFL, SIG_IGN};
use libc::{signal, raise};

fn main() {
    unsafe {
        // Ignore the SIGTERM signal
        signal(SIGTERM, SIG_IGN);
        // no effect since SIGTERM is ignored
        raise(SIGTERM);
    }

    println!("ok");

    unsafe {
        // Reset SIGTERM to its default
        signal(SIGTERM, SIG_DFL);
        // SIGTERM takes effect - terminating the program
        raise(SIGTERM);
    }

    // never reached - program terminated before here
    println!("not ok");
}
