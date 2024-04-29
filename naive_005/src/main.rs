use std::thread;
use std::time::{Duration};

fn main() {

    loop {
        println!("Still loopin'...");
        thread::sleep(Duration::from_secs(5));
        // deal with Ctrl-C
    }

}
