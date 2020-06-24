use toml::Value;

fn main() {
    let value = "foo = 'bar'".parse::<Value>().unwrap();

    // assert_eq!(value["foo"].as_str(), Some("bar"));

    println!("value ===>{}", value);
    println!("value.foo ===>{}", value["foo"]);
}


// demo 2
/*
use toml::Value;

fn main() {
    let value_str = r"#
        foo='bar'
        hello='world'
    #";

    let value=value_str.parse::<Value>().unwrap();

    // let value = "foo = 'bar'".parse::<Value>().unwrap();


    println!("value str ===>\n{}", value);
    println!("value.foo ===>{}", value["foo"]);
    println!("value.hello ===>{}", value["hello"]);
}

*/