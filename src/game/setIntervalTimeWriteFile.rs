// 定时器写入文件
use std::fs;
//fn write_file() -> std::io::Result<()> {
//    fs::write("foo.txt","哈哈哈")?;
//    Ok(())
//}

fn write_file(){
    let _ = fs::write("foo.txt", "哈哈哈");
    //此处不能用分号！
}

fn main() {
    println!("hello world");
    write_file();
}


//fn createFile(){
//    let file= std::fs::File::create("data.txt").expect("创建文件失败");
//    println!("文件{:?}",file);
//}


//use std::fs;
//
//fn main() -> std::io::Result<()> {
//    fs::write("foo.txt", b"Lorem ipsum")?;
//    Ok(())
//}
