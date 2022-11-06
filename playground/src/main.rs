use std::{thread, time};

fn main() {
    let one_second = time::Duration::from_secs(1);
    loop {
        println!("Hello, world!");
        thread::sleep(one_second);
    }
}