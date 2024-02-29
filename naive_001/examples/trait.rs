use std::convert::From;

#[allow(dead_code)]
#[derive(Debug, Default)]
struct MyData {
    a: MyConvertedString,
    b: String,
    c: MyConvertedString,
    d: MyConvertedString,
    e: String,
    f: String,
    g: MyConvertedString,
    h: MyConvertedString,
    i: String,
    j: MyConvertedString,
}

#[derive(Debug, Default)]
struct MyConvertedString ( String );

impl From<&str> for MyConvertedString {
    fn from(item: &str) -> Self {
        MyConvertedString ( item.to_uppercase() )
    }
}


fn main() {

    let my_string = MyData {
        a: MyConvertedString::from("a_String"),
        b: "b_String".to_string(),
        c: MyConvertedString::from("c_String"),
        d: MyConvertedString::from("d_String"),
        e: "e_String".to_string(),
        f: "f_String".to_string(),
        g: MyConvertedString::from("g_String"),
        h: MyConvertedString::from("h_String"),
        i: "i_String".to_string(),
        j: "j_String".into(),
    };

    println!("my_string after init: {:?}", my_string);

}

