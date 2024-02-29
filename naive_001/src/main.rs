//! Berlin Rust Hack and Learn
//! NAIVE: Novice And Intermediate Vocational Exercises
//! 
//! 2024-02-15: NAIVE session #001
//! 
//! See README.md for more information.

#[allow(dead_code)]
#[derive(Debug, Default)]
struct MyData {
    a: String,
    b: String,
    c: String,
    d: String,
    e: String,
    f: String,
    g: String,
    h: String,
    i: String,
    j: String,
}

fn main() {

    let mut my_string = MyData {
        a: "a_String".to_string(),
        b: "b_String".to_string(),
        c: "c_String".to_string(),
        d: "d_String".to_string(),
        e: "e_String".to_string(),
        f: "f_String".to_string(),
        g: "g_String".to_string(),
        h: "h_String".to_string(),
        i: "i_String".to_string(),
        j: "j_String".to_string(),
    };

    println!("my_string after init: {:#?}", my_string);
    
    my_string.c = string_conversion(&my_string.c);
    my_string.d = string_conversion(&my_string.d);
    my_string.g = string_conversion(&my_string.g);
    my_string.h = string_conversion(&my_string.h);
    my_string.j = string_conversion(&my_string.j);
    
    println!("my_string after conversion: {:#?}", my_string);

}

fn string_conversion (s: &str) -> String {
    s.to_uppercase()
}
