use std::io;
//io 输入输出库,io库来自std标准库
use rand::Rng;

// rust 将prelude模块少量的类型引入到每个程序的作用域中。如果不在，需要`use`语句显示地引入到作用域
//
fn main() {
    println!("hello world");
    random()
}

//

// 手动输入显示数字
fn guess() {
    let mut guess = String::new(); //?干嘛的，空字符串,这个new干嘛的？
    println!("请输入：{}", guess);
    io::stdin().read_line(&mut guess)   //&mut 干嘛的？
        .expect("读取行失败"); // expect 函数
    println!("你猜的，{}", guess);
}


