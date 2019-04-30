// literals and operators
/**
integers 1
float 1.2
characters 'a'
string 'abc'
boolean true/false
unit type 元类型 `()`
unit type
整数使用前缀表示
    0x
    0o
    0b

Integer addition 整数运算
*/
fn main(){
    println!("1+2={}",1u32+2); //1u32，表示1 u32的吗？ 3
    println!("1+2={}",1i32+2); //1u32，表示1 u32的吗？ 1u32-2 报错 3
    println!("1-2={}",1i32-2); //负号整数 -1

    // Short-circuiting boolean logic
    println!("true AND false is: {}",true&&false);//false
    println!("true OR false is: {}",true||false);//true
    println!("NOT true is :{}",!true);//false
    println!("NOT false is :{}",!false);//true

    // Bitwise operations 按位操作符
    /*
        __________
        1 & 1  = 1 全为真则真
        1 & 0  = 0
        0 & 1  = 0
        0 & 0  = 0
        __________
        1 | 1 = 1 存在一个为真则全为真
        1 | 0 = 1
        0 | 1 = 1
        0 | 0 = 0
        __________
        1 ^ 1 = 0 相同0 相反为 1
        0 ^ 0 = 0
        0 ^ 1 = 1
        1 ^ 0 = 1
        __________

    */
    // println!("0011 AND 0101 is {:04b}",0b0011 & 0b0101); //0001
    println!("0011 AND 0101 is {:04b}",0b0011u32 & 0b0101);//0001  {:04b}啥意思 表示4个长度的二进制吗
    println!("0011 OR 0101 is {:04b}",0b0011u32|0b0101);// 0111
    println!("0011 XOR 0101 is {:04b}",0b0011u32 ^ 0b0101);// 0110

    println!("1<<5 is {}",1u32 <<5 );//左移动5位
    println!("0x80 >> 2 is 0x{:x}",0x80u32>>2);
    println!("x的hex:{:x}",99);//0x64
    println!("99>>2:{}",99>>2);//24
    println!("99<<2:{}",99<<2);//396

    // Use underscores to improve readability!
    // 下划线 改善可读性
    println!("One million is written as {}",1_000_35_352u32);//100035352

}

