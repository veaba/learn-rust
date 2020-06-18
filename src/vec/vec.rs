use std::clone::Clone;

/**
@desc Rust 概念：所有权
@url http://wiki.jikexueyuan.com/project/rust-1.7/introduce.html
*/

fn main(){
	let mut  x= vec!["hello","world"];
    let y = &x[0].clone();
    x.push("xxx");

    let a = arg_array(arg.clone());
    println!("arg result==?,{:?}", a);

}


pub fn arg_array(arg: String) -> Vec<&'static str> {
    println!("入参=>{}", arg);
    let array = ["das", "das","dsad"];
    println!("len =>{}",array.len());
    let mut vec = Vec::with_capacity(array.len());
    for i in 0..array.len(){
        vec.push(array[i])
    }
    return vec;
}