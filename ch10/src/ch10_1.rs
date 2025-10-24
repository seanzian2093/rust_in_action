use std::{thread, time};

pub fn main() {
    let start = time::Instant::now();
    let handler = thread::spawn(|| {
        let pause = time::Duration::from_millis(300);
        // thread::sleep(pause.clone());
        thread::sleep(pause);
    });

    // join() instructs the OS to defer the calling thread until other thread finishes
    handler.join().unwrap();

    let finish = time::Instant::now();

    println!("{:02?}", finish.duration_since(start));
}
