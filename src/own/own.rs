//如果有引用它复制的变量，则会激活。不会销毁
fn main(){
	let a =20;
    let b = a;
//    println!("{}-{}",a,b);//这样不会被回收？
//    println!("{}",a)
    println!("{}",a);
    println!("{}",b);
}

// 给另外一个变量复制，也没有引用，报错
//fn main(){
//    let a =20;
//    let b = a;
//    println!("{}",a);
//}