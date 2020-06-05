use std::fs;
use std::env;
fn main() {
    let path = fs::read_dir("static2");

    // let path = fs::read_link("a.txt");
    println!("{:?}", path);

    // Err(Os { code: 3, kind: NotFound, message: "系统找不到指定的路径。" })
    // Ok(ReadDir("static"))

    println!("{:?}",env::current_dir())

    let path_success = fs::read_dir("static");
    let path_error = fs::read_dir("static2");

    // let path = fs::read_link("a.txt");
    println!("{:?}", path_success.is_ok());// true
    println!("{:?}", path_error.is_ok()); //false
}
