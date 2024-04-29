
// cargo add tokio -F macros,rt-multi-thread,signal,time
// cargo add tokio-util -F rt
use tokio_util::sync::CancellationToken;
use std::sync::{Arc, Mutex};

// an example data structure to share between tasks
#[derive(Debug, Default)]
struct Shared {
    counter: u32,
    return_code: i32,
}

// define the max timeout we give the process to shut down before force quitting
const SHUTDOWN_TIMEOUT_IN_SECONDS: u64 = 2;

async fn write_data(shared: Arc<Mutex<Shared>>, token: CancellationToken) {

    // Shall not cause data corruption
    loop {

        // check if task needs to quit
        if token.is_cancelled() {
            // The token was cancelled, task can shut down
            println!("  Writing: CANCELED");
            return;
        }

        // writing task which should not be interrupted
        {
            println!("  Writing: START writing data");
            tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;

            // acquire MUTEX lock
            {
                let mut data = shared.lock().unwrap();
                data.counter += 1;
            }
            // release lock by leaving the scope

            println!("  Writing: END writing data");
        }

        // check again if task needs to quit
        if token.is_cancelled() {
            // The token was cancelled, task can shut down
            println!("  Writing: CANCELED");
            return;
        }

        // add possible condition to set return code on failed action
        if false {
            let mut data = shared.lock().unwrap();
            data.return_code = 1;
        }

        // some additional task to perform
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

/* copied from this post: https://stackoverflow.com/a/77591939 */

#[cfg(unix)]
async fn wait_for_shutdown(token: CancellationToken) -> i32 {
    use tokio::signal::unix::{signal, SignalKind};

    // Infos here:
    // https://www.gnu.org/software/libc/manual/html_node/Termination-Signals.html

    // more possible signals: https://github.com/tokio-rs/tokio/blob/master/tokio/src/signal/unix.rs#L68
    let mut signal_terminate = signal(SignalKind::terminate()).unwrap();
    let mut signal_interrupt = signal(SignalKind::interrupt()).unwrap();

    tokio::select! {
        _ = signal_interrupt.recv() => {
            println!("Received SIGINT (Ctrl+C)");
            token.cancel();
            1
        },
        _ = signal_terminate.recv() => {
            println!("Received SIGTERM.");
            token.cancel();
            1
        },
        _ = token.cancelled() => {println!("Received Cancellation Token"); 0 }
    }
}

#[cfg(windows)]
async fn wait_for_shutdown(token: CancellationToken) -> i32 {
    use tokio::signal::windows;

    // Infos here:
    // https://learn.microsoft.com/en-us/windows/console/handlerroutine

    // https://github.com/tokio-rs/tokio/blob/master/tokio/src/signal/windows.rs
    let mut signal_c = windows::ctrl_c().unwrap();
    let mut signal_break = windows::ctrl_break().unwrap();
    let mut signal_close = windows::ctrl_close().unwrap();
    let mut signal_shutdown = windows::ctrl_shutdown().unwrap();

    tokio::select! {
        _ = signal_c.recv() => { println!("Received CTRL_C."); 1 },
        _ = signal_break.recv() => { println!("Received CTRL_BREAK."); 1 },
        _ = signal_close.recv() => { println!("Received CTRL_CLOSE."); 1 },
        _ = signal_shutdown.recv() => { println!("Received CTRL_SHUTDOWN."); 1 },
        _ = token.cancelled() => { println!("Received Cancellation Token"); 0 },
    }
}

#[tokio::main]
async fn main() {

    // adds token to broadcast cancellation
    let token = CancellationToken::new();

    // add tracker to shut down gracefully
    let tracker = tokio_util::task::TaskTracker::new();

    // initialize shared object
    let shared = Arc::new(Mutex::new(Shared::default()));

    // start
    println!("Starting");

    // "create" temp file
    println!("Creating temp file");

    // async function writing data
    let write_shared_copy = shared.clone();  // the Arc needs to be cloned in order to acquire a lock
    let write_token = token.clone(); // the token also needs to be cloned
    tracker.spawn(async move {
        write_data(write_shared_copy, write_token.clone()).await;
        if !write_token.is_cancelled() {
            println!("  Writing: command finished: send cancel");
            write_token.cancel();
            println!("  Writing: command finished: after cancel");
        }
    });

    // async function polling data
    let polling_token = token.clone();
    tracker.spawn(async move {
        let mut counter: i32 = 0;
        let max_read = 30;
        println!("  Polling: worker");
        loop {
            println!("  Polling: start poll");
            tokio::select! {
                // Step 3: Using cloned token to listen to cancellation requests
                _ = polling_token.cancelled() => {
                    // The token was cancelled, task can shut down
                    println!("  Polling: CANCELED");
                    return;
                }
                _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {
                    // Long work has completed
                    println!("  Polling: sleep finished");
                }
            }
            counter += 1;

            println!("  Polling: #{counter} done");

            if counter >= max_read {
                println!("  Polling: Finished Reading (counter at max)");
                polling_token.cancel();
                return;
            }
        }
    });

    tracker.close();

    // intermediate stuff to do
    println!("Hello, world!");

    // handle all signals and graceful shutdown
    let mut return_code = wait_for_shutdown(token.clone()).await;

    println!("Shutdown initiated");

    // add a timeout to force quite if shutdown takes way longer then expected
    let emergency_token = CancellationToken::new();
    let emergency_token_clone = emergency_token.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(SHUTDOWN_TIMEOUT_IN_SECONDS)).await;
        emergency_token_clone.cancel();
    });

    // handle double Ctrl+C by user
    // force exit if shutdown process keeps hanging
    tokio::spawn(async move {
        if wait_for_shutdown(emergency_token.clone()).await == 0 {
            println!("Shutdown timed out. QUIT");
        } else {
            println!("Double Ctrl+C by user/system. QUIT");
        }
        std::process::exit(1);
    });

    // wait for all tasks to finish
    tracker.wait().await;

    // cleanup the temp file
    println!("Cleaning up temp file");

    // print data from shared object
    println!("Result: {shared:#?}");

    // check if task returned an issue on regular shutdown
    if return_code == 0 {
        let data = shared.lock().unwrap();
        return_code = data.return_code;
    }

    // exit
    std::process::exit(return_code);
}