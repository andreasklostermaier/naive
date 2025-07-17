fn main() {
    let i = 42u32;
    let s = "aString".to_string();
    let o = Some(42.0f64);

    println!("i is of type {}", std::any::type_name_of_val(&i));
    println!("s is of type {}", std::any::type_name_of_val(&s));
    println!("o is of type {}", std::any::type_name_of_val(&o));
}
