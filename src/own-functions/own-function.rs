/**
*@desc 字符 \d \r \p  String::from怎么处理
*@如果让fn 返回一个字符，怎么出来
*
*/fn main(){
	let s =String::from("哇哈哈\n");//啥意思？
    take_ownership(s);
    let ch = '1';
    let a="dsad";
    str(a);
    move_copy(ch);
    let tt = '啦';
    move_copy(tt);
    println!("main:{}",ch);
    let bbb = "a_return";
    println!(a_return())
}
fn take_ownership(str:String){
    println!("take_ownership:{}",str)
}
fn a_return(){
    let str1 = "stssad";
    return str1;
}
fn str(str:&str){
    println!("str:{}",str)
}
fn move_copy(c:char){
    println!("move_copy:{}",c)
}