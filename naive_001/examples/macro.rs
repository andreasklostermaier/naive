#[macro_export]
macro_rules! field_converter {
    // The macro takes the object name and a list of fields to converse.
    ($obj:expr, $($field:ident),*) => {
        $(
            $obj.$field = string_conversion(&$obj.$field);
        )*
    }
}


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
    
    field_converter![my_string,a,c,d,g,h,j];
    
    println!("my_string after conversion: {:#?}", my_string);

}

fn string_conversion (s: &str) -> String {
    s.to_uppercase()
}
