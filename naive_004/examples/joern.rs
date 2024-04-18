use std::path::PathBuf;

fn main() {

    let a = "/home/naive/"; // String_literal
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
    
    let x = b.to_string() + &c.to_string();
    let pb: PathBuf = [a.to_string(), x, d.to_string(), e].iter().collect();
    println!("{:#?}", pb);
    assert_eq!("/home/naive/session_42/README.txt", pb.to_string_lossy());
}

/*
solution Jörn:
    let mut file = String::from(b);
    file.push(c);
    file.push_str(&d.to_string());
    let pb: PathBuf = PathBuf::from(a).join(file).join(e);
    println!("{:#?}", pb);
    assert_eq!("/home/naive/session_42/README.txt", pb.to_string_lossy());

*/ 