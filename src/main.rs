use serde_json::{Result, Value};

// #[derive(Debug)]
// pub struct TestStruct {
//     data_int: u8,
//     data_str: String,
//     data_vector: Vec<u8>,
//     school: School,
// }
//
// struct School {
//     name: String
// }

fn main() {
    let object = json_to_enum().unwrap();
    println!("{:#?}", object); // TestStruct` cannot be formatted using `{:?}`
    println!("=dsadd=>{}", object["name"]);

    match object {
        _name => println!("dad====>{}",_name),
    }
}


// JSON转Value枚举值
fn json_to_enum() -> Result<Value> {
    let json = r#"
        {
            "name":"asjdsak",
            "age":30,
            "type":true
        }
    "#;
    let v: Value = serde_json::from_str(json)?;
    println!("name===>{}", v["name"]);
    println!("==>{:?}", v);
    Ok(v)
}
