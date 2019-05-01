#[derive(Debug)]//TODO 干嘛的？
// <'a>是干嘛的？TODO
struct Person<'a>{
    name:&'a str,
    age:u8
}
fn main(){
	let name= "JoGel";
    let age = 26;
    let peter =Person{name,age};
    //:#? 是干嘛的
    println!("{:#?}",peter);
}
