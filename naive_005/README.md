# Berlin Rust Hack and Learn: The NAIVE<sup>*</sup> Sessions

## 2024-04-25: NAIVE<sup>*</sup> session #005 – Ctrl-C

<sup>*</sup>Novice And Intermediate Vocational Exercises

### Challenge description

Our challenge is to catch the CTRL-C signal in a running Rust application in such a way, that we can decide to execute some code and quit gracefully. We are looking for a solution that works on Nix and Windows, which might be difficult, as Windows does not have Nix-like signals, but rather Control Handlers. For this challenge we only want to deal with the CTRL-C interrupt. A future excercise could be to find a way to deal with any signals/Control Handlers.


### Solutions

Initial research indicated to us, that any solution can only be a form of concurrency: some code has to listen for the signals, some other code has to do the actual work. We came up with the following solutions:

#### 1. CTRLC crate

There is a readymade crate `ctrlc` on crates.io, just for our purpose. Concurrency is done with threads in `ctrlc`. In our small example we spawn the signal listener with `ctrlc::set_handler` and afterwards we start a while loop that does the actual work (in our case it just prints status text and waits for 5 seconds). We use a static variable `EXIT` to communicate the reception of the CTRL-C event betweem the threads. The solution works on NIX and Windows. But the solution is just a rough boilerplate - its main problem is, that the CTRL-C event is only checked once per loop iteration - it might take 5 seconds before our application responds to it.

```bash
cargo run --example ctrlc
```

#### 2. Tokio

The second solution uses Tokio to spawn two green threads with the `tokio::select` macro. The code looks pretty clean and the CTRL-C event is handled immediately. Tokio implements `signal::ctrl_c()` for NIX and Windows and compiles according to the target. Also see the [Tokio Signal Module documentation](https://docs.rs/tokio/latest/tokio/signal/index.html).

```bash
cargo run --example tokio
```


#### 3. A more complete Tokio solution

In a follow up effort, contributor Ricardo provided a much more elaborate template for a signal listener, that not only works cross-platform, but also handles all kind of interruption signals. It is also based on green threads with Tokio. His code is available at 
[Github](https://github.com/bb-Ricardo/rust-tokio-signal-handling/tree/main) and provided as example here with kind permission:

```bash
cargo run --example ricardo
```

