// use toml::Value;

// fn main() {
//     let value_str = r"#
//         foo='bar'
//         hello='world'
//     #";

//     let value=value_str.parse::<Value>().unwrap();

//     // let value = "foo = 'bar'".parse::<Value>().unwrap();

//     println!("value str ===>\n{}", value);
//     println!("value.foo ===>{}", value["foo"]);
//     println!("value.hello ===>{}", value["hello"]);
// }
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug)]
struct IpConfig {
    name: Option<String>,
    ip: Option<String>,
    port: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Conf {
    ip_config: Option<Vec<IpConfig>>,
    security: Option<SecurityModule>,
}

#[derive(Deserialize, Debug)]
struct SecurityModule {
    key: Option<String>,
    cert: Option<String>,
}

fn main() {
    let file_path = "config.toml";
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception:{}", file_path, e),
    };
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(e) => panic!("Error Reading file: {}", e),
    };
    let config: Conf = toml::from_str(&str_val).unwrap();

    println!("==>{:#?}", config);

    // println!("security===>{:?}", config.security.unwrap());
    println!("ip_config===>{:?}", config.ip_config.unwrap());
    // println!("unwrap security===>{:?}", config.security.unwrap());
    println!("security key===>{:?}", config.security.unwrap().key.unwrap());

    /*
    for x in config.ip_config.unwrap() {
        println!("{:?}", x);
    }*/
}
