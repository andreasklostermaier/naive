use std::path::PathBuf;

fn main() {

    let a = "/home/naive/";
    let b = "session";
    let c = '_';
    let d = 42;
    let e = "README.txt".to_string();
    
    // create and print a PathBuf variable for the following path:
    //
    // /home/naive/session_42/README.txt
    // 
    // The provided variables have to be used!!
    // DO NOT USE THE "format!" macro!!
    let mut s = b.to_string();
    s.push(c);
    s.push_str(&d.to_string());
    
    let pb: PathBuf = [a, b, s.as_str(), e.as_str()].iter().collect();
    
    println!("{:#?}", pb);
    
}