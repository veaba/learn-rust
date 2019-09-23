// 参考学习地址：https://www.twle.cn/c/yufei/rust/rust-basic-file-input-output.html
//use std::thread;
//use std::time::Duration;
//use std::fs::OpenOptions;
//use std::io::Write;
use std::fs;

fn main() {
    let x = 9999;
//    while x < 1000 * 3600 * 24 {
//        println!("{},{}", x, "哈哈哈哈");
//        x += 1;
//        // 写入文本
//        fs_write(x);
////        fs_append(x);
//        thread::sleep(Duration::from_millis(1000));
//    }
    let addr= &x as *const i32 as usize;
    println!("addr: 0x{:X}",addr);


    fs_write(x);

    println!("{},{}", x, "========= 跑了二十个小时，停止咯 ========");
}


// 单纯的写入文本，会覆盖掉
fn fs_write(x:i32) {
    let _ = fs::write("main1.txt", x.to_string());
}

// 追加的方式写入文本

//fn fs_append(x:i32){
//    println!("哈哈哈哈============");
////    let mut file=OpenOptions::new().append(true).open("main1.txt").expect("异常");
////    file.write_all((x.to_string()+"：写文件呀\n").as_bytes()).expect("write_all 错误");
//}
