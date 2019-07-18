
fn main() {
    tuple_fn()
}



fn tuple_fn(){
    let long_tuple =(
        1u8,2u16,3u32,4u64,
        -1i8,-2i16,-3i32,-4i64,
        0.1f32,0.2f64,'a',true);
    println!("{:?}",long_tuple);
    println!("{}",long_tuple.4);


}
