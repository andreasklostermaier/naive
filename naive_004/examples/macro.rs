use std::path::PathBuf;

macro_rules! make_path {
    ( $( $x:expr ),* ) => {
        [
            $(
                $x.to_string(),
            )*
        ].into_iter().collect::<PathBuf>()
    };
}

#[tokio::main]
async fn main() {
    let p = make_path!("/home/naive", "session", '_', 42, "README.txt");
    dbg!(p);
    }