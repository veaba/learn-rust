fn main(){
	let s =String::from("SDUSAJDOISAJ IUJODSJAI ");//啥意思？
    take_ownership(s);
    let ch ='a';
    move_copy(ch);
    println!("main:{}",ch)
}
fn take_ownership(str:String){
    println!("take_ownership:{}",str)
}
fn move_copy(c:char){
    println!("move_copy:{}",c)
}