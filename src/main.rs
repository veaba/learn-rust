// extern crate rustc_serialize;
//
// use rustc_serialize::json;
//
// // Automatically generate `RustcDecodable` and `RustcEncodable` trait
// // implementations
// #[derive(RustcDecodable, RustcEncodable)]
// #[derive(Debug)]
// pub struct TestStruct {
//     data_int: u8,
//     data_str: String,
//     data_vector: Vec<u8>,
//     data_school: School,
// }
//
// struct School {
//     school_name: String,
//     location: String,
// }
//
// fn main() {
//     // 初始化TestStruct
//     let object = TestStruct {
//         data_int: 1,
//         data_str: "homura".to_string(),
//         data_vector: vec![2, 3, 4, 5],
//         data_school: School {
//             school_name: "primary school".to_string(),
//             location: "HK".to_string(),
//         },
//     };
//
//     // 将TestStruct转意为字符串
//     let encoded = json::encode(&object).unwrap();
//     println!("Struct nesting struct 转为 字符串==> {}", encoded);
//     // Deserialize using `json::decode`
//     // 将json字符串中的数据转化成TestStruct对应的数据，相当于初始化
//     let decoded: TestStruct = json::decode(&encoded).unwrap();
//     println!("json字符串转为struct data_vector==>{:?}", decoded.data_vector);
//     println!("json字符串转为struct==>{:#?}", decoded);
// }
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
