/**
@desc 铸造
@Rust provides no implicit type conversion (coercion) between primitive types. But, explicit type conversion (casting) can be performed using the as keyword.
@Rust 没有提供原始类型之间的隐式类型转换（强制转换），但是，显示类型转换可以通过关键字 `as` 实现
@Rules 遵从C的规则，除非C有未定义的行为，完整类型之间的所有类型行为在Rust中都有很好的定义
*/


// Suppress all warnings from casts which overflow.
// 禁止所有警告

// literals  文字
// implicit 隐式
// explicit 显式
//#![allow(overflowing_literals)]

fn main(){
	let decimal = 3.14159265326898956565156;
    // let integer:u8 = decimal;

    // 显式转换explicit conversion
    let integer =decimal as u8;

    println!("decimal:{}",decimal); //3.141592653268987
    println!("integer:{}",integer); // 3

    let character = integer as char;

    println!("character:{}",character);//╚


    println!("A吗？:{}",48u8 as char);//╚
    println!("A吗？:{}",60u8 as char);//╚
    println!("A吗？:{}",36u8 as char);//╚
    println!("A吗？:{}",40u8 as char);//╚

}
