use rustc_serialize::json;
#[derive(Debug)]
pub struct TestStruct {
    data_int: u8,
    data_str: String,
    data_vector: Vec<u8>,
    school: School,
}

struct School {
    name: String
}

fn main() {
    let object = TestStruct {
        data_int: 1,
        data_str: "homura".to_string(),
        data_vector: vec![2, 3, 4, 5],
        school: School {
            name: "浙江".to_string()
        },
    };
    println!("{:#?}", object) // TestStruct` cannot be formatted using `{:?}`
}
