extern crate rustc_serialize;

use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug)]
struct Profile {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn main() {
    let json_str = r#"
    {
        "name":"veaba",
        "age":26,
        "phones":[
            "110",
            "120",
            "119"
        ]
    }
    "#;
    let profile: Profile = json::decode(&json_str).unwrap();
    println!("json to struct==>{:#?}", profile);
}
