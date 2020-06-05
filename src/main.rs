use std::fs;
// use std::env;
fn main() {
    let path_success = fs::read_dir("static");
    let path_error = fs::read_dir("static2");

    // let path = fs::read_link("a.txt");
    println!("{:?}", path_success.is_ok());
    println!("{:?}", path_error.is_ok());

}
