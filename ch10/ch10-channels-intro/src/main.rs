#[macro_use]
extern crate crossbeam;

use std::thread;
use crossbeam::channel::unbounded;

fn main() {
    let (tx, rx) = unbounded();
    thread::spawn(move || {
        tx.send("Hello, world from ch10-channels-intro!").unwrap();
    });

    // select! macro simplifies receiving messages
    select! { recv(rx) -> msg => println!("{:?}", msg),}
}
