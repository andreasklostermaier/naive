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

#[allow(dead_code)]
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

    string_conversion(vec![
        &mut my_string.a,
        &mut my_string.c,
        &mut my_string.d,
        &mut my_string.g,
        &mut my_string.h,
        &mut my_string.j,
    ]);

    println!("my_string after conversion: {:#?}", my_string);

}


fn string_conversion(elements: Vec<&mut String>) {
    elements.into_iter().for_each(|element| {
        let converted = element.to_uppercase();
        *element = converted;
    });
}
