mod thread;
mod tokio_unblock;
mod tokio_block_in_place;
mod tokio_block;
fn main() {

    println!("std thread results:");
    thread::run();

    println!("tokio unblock results:");
    tokio_unblock::run();

    println!("tokio block in place results:");
    tokio_block_in_place::run();

    println!("tokio block results:");
    tokio_block::run();
}