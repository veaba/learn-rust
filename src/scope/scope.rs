/**
@desc 作用域
*/
fn main(){
	let x =999;
    {
        let y =22333;
        println!("x scope {}",x);
        println!("y scope {}",y);
    }
    println!("x global {}",x);
//    println!("y global {}",y);
}
