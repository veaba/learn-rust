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
use std::fmt;
use serde_derive::Serialize;

#[derive(Serialize)]
struct Config {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}

#[derive(Serialize)]
struct Keys {
    github: String,
    travis: Option<String>,
}

impl fmt::Display for Config{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", &self)
    }
}

fn main() {
    let config = Config {
        ip: "127.0.0.1".to_string(),
        port: None,
        keys: Keys {
            github: "xxxxxxxxxxxxxxxxx".to_string(),
            travis: Some("yyyyyyyyyyyyyyyyy".to_string()),
        },
    };
    // let x =String::from(config);
    println!("struct to string===>{}", config.to_string());

    let toml = toml::to_string(&config).unwrap();

    println!("toml string==>{}", toml)
}
