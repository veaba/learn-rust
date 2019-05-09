/**
@desc 字符串
*/

fn main(){
    println!("一个函数返回一个字符串：{}",return_a_string());
}

fn return_a_string()->(&'static str){
    let str = "哇哈哈哈";
    return  str
}
