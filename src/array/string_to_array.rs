fn main() {
    // let a = "config=config.toml";
    let b = "config=config.toml port=80 start stop";

    let c=b.split(" ");
    let mut vec_array= vec![];
    for x in c{
        println!("==>{}",x);
        vec_array.push(x);
    }
    println!("vec_array==>{:?}",vec_array); // ["config=config.toml", "port=80", "start", "stop"]
}
