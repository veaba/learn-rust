use std::io;
fn main(){
    println!("hello world");
    let mut guess =String::new(); //?干嘛的，空字符串
    println!("请输入：{}",guess);

    io::stdin().read_line(&mut guess)
        .expect("读取行失败");
    println!("你猜的，{}",guess);
}
