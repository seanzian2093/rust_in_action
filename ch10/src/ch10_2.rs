use std::{thread, time};

pub fn main() {
    let start = time::Instant::now();
    
    let handler1 = thread::spawn(|| {
        let pause = time::Duration::from_millis(300);
        // thread::sleep(pause.clone());
        thread::sleep(pause);
    });

    let handler2 = thread::spawn(|| {
        let pause = time::Duration::from_millis(300);
        // thread::sleep(pause.clone());
        thread::sleep(pause);
    });

    // join() instructs the OS to defer the calling thread until other thread finishes
    handler1.join().unwrap();
    handler2.join().unwrap();

    let finish = time::Instant::now();

    println!("{:02?}", finish.duration_since(start));
}
