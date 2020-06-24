extern crate rustc_serialize;

use rustc_serialize::json;

// Struct Parent
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
struct Parent {
    name: String,
    age: u8,
    children: Children,
}

// Struct Children
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
struct Children {
    name: String,
    age: u8,
    school: String,
}

fn main() {
    let person = Parent {
        name: "Li".to_string(),
        age: 35,
        children: Children {
            name: "Li's son".to_string(),
            age: 8,
            school: "primary".to_string(),
        },
    };
    println!("no children==>{:#?}", person); // 可以打印出来person

    let to_json_str = json::encode(&person).unwrap();
    println!("json str==>{}", to_json_str);
}
