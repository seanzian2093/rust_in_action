#[macro_use]
extern crate crossbeam;

use std::thread;
use crossbeam::channel::unbounded;

use crate::ConnectivityCheck::*;

#[derive(Debug)]
enum ConnectivityCheck {
    Ping,
    Pong,
    Pang,
}

fn main() {
    let n_msg = 3;

    // channel to send and receive requests
    let (requests_tx, requests_rx) = unbounded();
    // channel to send and receive responses
    let (responses_tx, responses_rx) = unbounded();

    // Send response according to what request is received
    thread::spawn(move || loop {
        match requests_rx.recv().unwrap() {
            Pong => eprintln!("unexpected Pong response"),
            Ping => responses_tx.send(Pong).unwrap(),
            // return means shut down the thread
            Pang => return,
        }
    });

    // Send out requests
    for _ in 0..n_msg {
        requests_tx.send(ConnectivityCheck::Ping).unwrap();
    }
    requests_tx.send(ConnectivityCheck::Pang).unwrap();

    // Receive responses
    for _ in 0..n_msg {
        select! {
            recv(responses_rx) -> msg => println!("{:?}", msg)
        }
    }
}
