use std::clone::Clone;

/**
@desc Rust 概念：所有权
@url http://wiki.jikexueyuan.com/project/rust-1.7/introduce.html
*/

fn main(){
	let mut  x= vec!["hello","world"];
    let y = &x[0].clone();
    x.push("xxx");

}
