use serde::{Deserialize, Serialize};
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
    println!("object{:#?}", object); // TestStruct` cannot be formatted using `{:?}`
    println!("=dsadd=>{}", object["name"]);

    match object {
        name => println!("dad====>{}", name),
    }

    let person = parsing_json_to_struct().unwrap();
    println!("main person.name==>{}", person["name"])
    // println!("main person==>{:#?}", person["name"])
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
    println!("==>{:?}", v); // Json to enum
    Ok(v)
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

// 将JSON解析为强类型数据结构
fn parsing_json_to_struct() -> Result<Person> {
    let data = r#"
    {
        "name":"Veaba",
        "age":26,
        "phones":[
            "110",
            "120",
            "119"
        ]
    }
    "#;
    let person: Person = serde_json::from_str(data)?;
    println!("data==>{:#?}", person);
    Ok(person)
}
