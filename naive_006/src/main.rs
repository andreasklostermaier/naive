fn main() {

    let i = 42u32;
    let s = "aString".to_string();
    let o = Some(42.0f64);

    println!("i is of type {}", get_type(&i));
    println!("s is of type {}", get_type(&s));
    println!("o is of type {}", get_type(&o));

}

fn get_type<T>(var: &T) -> String {

}