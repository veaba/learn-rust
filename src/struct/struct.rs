#[derive(Debug)]//必须这加这一段，否则无法使用 {:#?}打印struct
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


/*
struct Person {
    name: String,
    age: u8,
}
fn main() {
    let name = "JoGel";
    let age = 26;
    let peter = Person {
        name: String::from(name),
        age,
    };

    //:#? 是干嘛的
    println!("{:#?}", peter);
}


*/
