
//fn main(){
//    // 类似 for(let i=0;i<=99;i++)的c语言迭代
//    for i in 1..99 {
//        println!("{}",i)
//    }
//    let mut res;
//    for i in 1..20 {
//        res=2*i;
//        println!("2*{}={}",i,res)
//    }
//}
//TODO 循环数组
//fn main(){
//	let res =["mango","apple","banana","litchi","watermelon"];
//    for i in res.iter() {
//        println!("{}",i)
//    }
//}
//TODO 循环对象
//
//fn main(){
////	let ob=({"xx":6,"oo":666});
//}

// 读取数组
//fn main(){
//	let res =["mango","apple","banana","litchi","watermelon"];
//    println!("{}",res[2])
//}

// iter 做条件

fn main(){
	let res =["mango","apple","banana","litchi","watermelon"];
    for i in res.iter() {
        if  i == "mango" {
            println!("{}",i);
        }

    }
}