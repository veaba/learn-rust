/**
* @desc 数组和切片
*/
use std::mem;//这是什么包
// This function borrows a slice
fn analyze_slice(slice:&[i32]){
    println!("元素[0]：{}",slice[0]);
    println!("长度：{}",slice.len());
}
fn main(){
	let xs:[i32;5]=[1,335,669,996,6];//定义数组的类型和长度
    // 所有元素到相同的值All elements can be initialized to the same value
    let ys:[i32;500]=[99;500];

    let zs:[i32;1]=[1];

    let ws:[&str;1]=["中国"];

    println!("第一个数组元素：{}",xs[0]);
    println!("第二个数组元素：{}",xs[1]);

    println!("xs数组:{:?}",xs);

    // println!("ys数组:{:?}",ys); 无法打印ys，可能是太长了

    //返回数组的长度 implement 实现
    println!("xs数组的长度：{}",xs.len());
    println!("ys数组的长度：{}",ys.len());

    // array occupies 占据
    println!("array xs的占据：{} bytes",mem::size_of_val(&xs));//20 bytes 一个元素四个字节 4x5的长度=20个 bytes
    println!("array ys的占据：{} bytes",mem::size_of_val(&ys));//2000 bytes
    println!("array zs的占据：{} bytes",mem::size_of_val(&zs));//4 bytes
    println!("array ws的占据：{} bytes",mem::size_of_val(&ws));//16 bytes，一个中文 8个字节

    // 数组切片
    analyze_slice(&xs);//为什么要加一个&呢？


    analyze_slice(&ys[0..4]);//分割数组，或者可以说是数组的一种解构方式

    // 绑定的索引会导致编译错误？啥意思~
    // out of bound indexing causes compile error
    // println!("{}",xs[5]); 错误
    println!("{}",xs[4]);//正常
}
