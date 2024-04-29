use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

static EXIT: AtomicBool = AtomicBool::new(false);

fn main() {
    ctrlc::set_handler(|| {
        println!("Hello from the handler");
        EXIT.store(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");

    while !EXIT.load(Ordering::Relaxed) {
        println!("Still loopin'...");
        thread::sleep(Duration::from_secs(5));
    }
}
