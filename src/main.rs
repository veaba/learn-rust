use std::io;
//io 输入输出库,io库来自std标准库
use rand::Rng;
//Rng 是一个trait特质，定义了随机数实现的方法
use std::cmp::Ordering;

//rust官方提供的随机库
fn main() {
    println!("hello world");
    random()
}


// 生成一个随机数
fn random() {
    println!("猜一个数字");
    //gen_range随机数生成器，此处是1到100之间的数
    let secret_number = rand::thread_rng().gen_range(1, 101);


    loop {
        println!("请输入一个数字：......");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("失败");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("你干嘛!?");
                continue
            }
        };


        println!("你猜的{}", guess);


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小"),
            Ordering::Greater => println!("太大"),
            Ordering::Equal => {
                print!("你赢了");
                break;
            }
        }

    }
}


// 手动输入显示数字
//fn guess() {
//    let mut guess = String::new(); //?干嘛的，空字符串,这个new干嘛的？返回一个String新的实例，
//    println!("请输入：{}", guess);
//
//    // read_line 从标准输入句柄获取用户输入，向read_line() 传递了一个参数 &mut guess
//    //
//    let x = io::stdin().read_line(&mut guess)   //&mut 干嘛的？ &表示这个参数是引用(reference)
//        .expect("读取行失败"); // expect 函数，此处出问题则会立即崩溃。后续会有从错误中恢复
//    println!("{}", x);//返回得到的一个usize 长度
//    println!("你猜的，{}", guess);
//}
