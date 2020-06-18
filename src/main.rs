extern crate regex;
fn main() {
    let a = String::from("config=config.toml ad");
    // let b = String::from("config=dsa=config.toml dasd "); //false
    // let c = "configoml "; //false
    // let a ="config=config.toml"; //true
    let x = is_compose_cmd(&a);
    println!("{:?}", x);
    // let reg= regex::Regex::new(r"^.*=.*$").unwrap();
    // let x =reg.is_match(b);
    // println!("{}",x);

    // for i in 0..3 {
    //     println!("{}",i)
    // };
}

fn is_compose_cmd(arg_string: &String) -> bool {
    let mut vec_array = vec![];
    let str_split = arg_string.split("=");
    for code in str_split {
        // vec_array.push(code);
        vec_array.push(code.trim());
    }
    println!("array===>{:?}", vec_array);
    if vec_array.len() != 2 {
        return false;
    }
    return true;
}
