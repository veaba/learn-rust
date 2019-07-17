/**

@desc 使用`use`使用声明，不需要手动范围界定
*/
enum Status{
    Rich,
    Poor
}
#[warn(dead_code)]
enum Work{
    Google,
    Microsoft,
    Alibaba=999
}
fn main(){
	// Explicitly `use` each name so they are available without

    // manual scoping 手册范围
    use Status::{Poor,Rich};

    // Automatically `use` each name inside `Work`.
    // 自动将每个枚举的名称转为:无值时候索引值，有值则打印值，且自动列出来Work枚举的全部，不用多写Work
    use Work::*;

    let status= Poor;
    let states = Rich;
    let work = Google;
    let worker =Microsoft;
    let taobao =Alibaba;//??


    println!("i32:{}",Work::Google as i32);//0
    println!("i32:{}",Work::Microsoft as i32);//1
    println!("i32:{}",Work::Alibaba as i32);//999

    //  println!("{:#?}",work); error
    //  println!("{:?}",work); error
    match status {
        Rich=>println!("有钱人"),
        Poor=>println!("穷逼")
    }
    match states {
        Rich=>println!("有钱人"),
        Poor=>println!("穷逼")
    }
    match work {
        Google=>println!("I am Google Company CEO"),
        Microsoft=>println!("我就是比尔盖茨"),
        _=>println!("我就是阿里巴巴与四十大盗1")
    }
    match worker {
        Google=>println!("I am Google Company CEO"),
        Microsoft=>println!("我就是比尔盖茨"),
        Alibaba=>println!("我就是阿里巴巴与四十大盗2")
    }
    match taobao {
        Google=>println!("I am Google Company CEO3"),
        Microsoft=>println!("我就是比尔盖茨3"),
        Alibaba=>println!("我就是阿里巴巴与四十大盗3{}")
    }
}
