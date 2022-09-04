use std::{
    fs::File,
    io::{Read, Write},
    time::Instant,
};
use tokio::task::{self, JoinHandle};

async fn compute() {
    let handles: Vec<JoinHandle<_>> = (0..1000)
        .map(|_| {
            tokio::spawn(async move {
                let mut buffer = [0; 10];

                task::block_in_place(move || {
                    let mut dev_urandom = File::open("/dev/urandom").unwrap();
                    dev_urandom.read(&mut buffer).unwrap();
                });
                
                task::block_in_place(move || {
                    let mut dev_null = File::create("/dev/null").unwrap();
                    dev_null.write(&mut buffer).unwrap();
                });
            })
        })
        .collect();
    for handle in handles {
        handle.await.unwrap();
    }
}

#[tokio::main]
pub async fn run() {
    // warmup
    compute().await;

    let before = Instant::now();
    for _ in 0usize..1000 {
        compute().await;
    }
    let elapsed = before.elapsed();
    println!(
        "{:?} total, {:?} avg per iteration",
        elapsed,
        elapsed / 1000
    );
}