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
    let mut b = b.to_owned();
    b.push(c);
    b.push_str(d.to_string().as_str());
    
    let pb: PathBuf = [a, b.as_str(), e.as_str()].iter().collect();
    
    println!("{:#?}", pb);
    
}