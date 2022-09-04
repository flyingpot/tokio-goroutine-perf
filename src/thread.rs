use std::time::Instant;
use std::{
    fs::File,
    io::{Read, Write},
    thread::JoinHandle,
};

fn compute() {
    let handles: Vec<JoinHandle<_>> = (0..1000)
        .map(|_| {
            std::thread::spawn(move || {
                let mut buffer = [0; 10];
                
                let mut dev_urandom = File::open("/dev/urandom").unwrap();
                dev_urandom.read(&mut buffer).unwrap();
                
                let mut dev_null = File::create("/dev/null").unwrap();
                dev_null.write(&mut buffer).unwrap();
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn run() {
    // warmup
    compute();

    let before = Instant::now();
    for _ in 0..1000 {
        compute();
    }
    let elapsed = before.elapsed();
    println!(
        "{:?} total, {:?} avg per iteration",
        elapsed,
        elapsed / 1000
    );
}