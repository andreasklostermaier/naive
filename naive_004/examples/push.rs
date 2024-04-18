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
    
    let pb: PathBuf = {
        let mut result = PathBuf::from(a);
        let mut dir = String::from(b);
        dir.push(c);
        dir.push_str(&d.to_string());
        result.push(dir);
        result.push(e);
        result
    };
    println!("{:#?}", pb);
    
}