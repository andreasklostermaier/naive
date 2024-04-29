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


fn main() {

    let a = "/home/naive/";
    let b = "session";
    let c = '_';
    let d = 42;
    let e = "README.txt".to_string();

    let mut foo = b.to_string();
    foo.push_str(&c.to_string());
    foo.push_str(&d.to_string());
    
    let p = make_path!(a, b, foo, e);
    dbg!(p);
}