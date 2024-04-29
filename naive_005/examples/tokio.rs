use std::time::Duration;
use tokio::{signal, time::sleep};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                println!("ctrl-c received!");
                break;
            }
            _ = sleep(Duration::from_secs(5)) => {
                println!("still looping...");
            }
        };
    }
    Ok(())

}
