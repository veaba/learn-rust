// if计算为整数值，如果是string 则抛错
//fn main(){
//    let a= if false {
//        1
//    }else {
//        2
//    };
//    println!("value a is :{}",a)
//}

fn main(){
    let a= if false {
        1
    }else {
        '2'
    };
    println!("value a is :{}",a)
}