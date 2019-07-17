/**
@desc 枚举也可以像 C语言一样的使用 enums

*/

enum Number{
    Zero,
    Four,
    Sex
}

#[warn(dead_code)]
enum Colors{
    Red=0xff0000,
    Green=0x00ff00,
    Blue=0x0000ff
}

// casting

fn main(){
    // can be cast as integers. 可以为整数
    println!("zero is {}",Number::Zero as i32);
    println!("One is {}",Number::Four as i32);
    println!("Three is {}",Number::Sex as i32);
M
    // roses
    println!("玫瑰: #{:06x}",Colors::Red as i32);
    println!("大海: #{:06x}",Colors::Blue as i32);
}
