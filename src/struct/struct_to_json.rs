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
    let profile = Profile {
        name: "Jobs".to_string(),
        age: 99,
        // phones: vec!["110", "120", "199", "144"],
        phones: vec!["110".to_string(), "120".to_string(), "199".to_string(), "144".to_string()],
    };
    let encode = json::encode(&profile).unwrap();
    println!("struct to json==>{}", encode);
}
