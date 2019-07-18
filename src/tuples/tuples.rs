/*
* tuples 元组 保存任何任意数量的值
* @todo 元组如何转为数组？

*/
// 元组可以用作function参数和返回值，入参是i32,Boolean、返回是Boolean、i32
fn reverse(pair:(i32,bool))->(bool,i32){
    let (integer,boolean)=pair;//解构
    (boolean,integer)//这段是干嘛？
}

#[derive(Debug)]
struct Matrix(f32,f32,f32,f32);

fn main(){
    let long_tuple =(
        1u8,2u16,3u32,4u64,
        -1i8,-2i16,-3i32,-4i64,
        0.1f32,0.2f64,'a',true);

    // 使用索引提取值，不能直接打印取到值，
    // TODO 如何循环取到全部？
    println!("第一个值{}",long_tuple.0);
    println!("第二个值 {}",long_tuple.3);
    println!("全部值：{:?}",long_tuple);

    //元组里面再使用元组
    //Tuples can be tuple members
    let tuple_of_tuples =((1u8,2u16,3u32),(4u64,-1i8),-2i16);
    println!("元组嵌套:{:?}",tuple_of_tuples);

    // 数组打印
    let long_array = [99,65,656,126,-1];
    println!("数组：{:?}",long_array);

    // todo 不能打印长元组？？ 报错
    // let too_long_tuple=(1,65,56,6,565,65,95,5,655,656,64,65,5,61,6,5656);
    // println!("too long tuple:{:?}",too_long_tuple);

     let pair =(1,true);
     println!("pair is  {:?}",pair);

     println!("逆序 的pair ：{:?}",reverse(pair));

    println!("{}",fool(1));//期待结果是1000

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses,
    println!("包含一个元素的元组：{:?}",(5u32,)); //(5,)
    println!("仅仅一个整数的元素：{:?}",(900u32));//(900)

    // tuples can be destructured to create bindings

    let tuple=(1,"hello",40.5,true);
    let (a,b,c,d)=tuple;//解构
    println!("{:?},{:?},{:?},{:?}",a,b,c,d);

    //矩阵
    let matrix = Matrix(2.1,9.3,4.9,4.2);
    println!("{:?}",matrix);//Matrix(2.1,9.3,4.9,4.2)


}


fn fool(num:i32)->(i32){
    return num+999//可以不要return
}
