
fn write_file(){
    let _ = fs::write("foo.txt", "哈哈哈");
    //此处不能用分号！
}

fn main() {
    println!("hello world");
    write_file();
}
