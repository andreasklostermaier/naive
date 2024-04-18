use std::path::PathBuf;

fn main() {

    let x = "/home/naive/";
    let c = "session";
    let a = '_';
    let b = 42;
    let d = "README.txt".to_string();
    
    // targetpath
    // /home/naive/session_42/README.txt
    
    // DO NOT USE THE "format!" Macro!!
    
    let mut foo = c.to_string();
    foo.push_str(&a.to_string());
    foo.push_str(&b.to_string());
    
    
    let pb: PathBuf = [&x, &foo[..], &d].iter().collect();
    println!("{:#?}", pb);
    
}

