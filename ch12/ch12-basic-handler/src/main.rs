#![cfg(not(windows))]
use std::time::{Duration};
use std::thread::sleep;
use libc::{SIGTERM, SIGUSR1};

static mut SHUT_DOWN: bool = false;

fn main() {
    register_signal_handler();

    let delay = Duration::from_secs(1);

    // from 1 to maximum of usize, which is 8
    for i in 1_usize.. {
        println!("{}", 1);

        unsafe {
            if SHUT_DOWN {
                println!("*");
                return;
            }
        }

        sleep(delay);

        let signal = if i > 2 {
            SIGTERM
        } else {
            SIGUSR1
        };

        unsafe {
            libc::raise(signal);
        }
    }
    unreachable!();
}

fn register_signal_handler() {
    // Modifying a mutable static is unsafe
    unsafe {
        libc::signal(SIGTERM, handle_sigterm as usize);
        libc::signal(SIGUSR1, handle_sigusr1 as usize);
    }
}

#[allow(dead_code)]
fn handle_sigterm(_signum: i32) {
    register_signal_handler();
    println!("SIGTERM received!");

    unsafe {
        SHUT_DOWN = true;
    }
}

#[allow(dead_code)]
fn handle_sigusr1(_signum: i32) {
    register_signal_handler();
    println!("SIGUSR1 received!");
}
