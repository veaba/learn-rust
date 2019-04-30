/**

@desc 使用`use`使用声明，不需要手动范围界定
*/
enum Status{
    Rich,
    Poor
}
#[warn(dead_code)]
enum Work{
    Google,
    Microsoft,
}
fn main(){
	// Explicitly `use` each name so they are available without

    // manual scoping 手册范围
    use Status::{Poor,Rich};

    // Automatically `use` each name inside `Work`.
    use Work::*;

    let status= Poor;
    let states = Rich;
    let work = Google;
    let worker =Microsoft;


    println!("i32:{}",Work::Google as i32);//0
    println!("i32:{}",Work::Microsoft as i32);//1

    //  println!("{:#?}",work); error
    //  println!("{:?}",work); error
    match status {
        Rich=>println!("有钱人"),
        Poor=>println!("穷逼")
    }
    match states {
        Rich=>println!("有钱人"),
        Poor=>println!("穷逼")
    }
    match work {
        Google=>println!("I am Google Company CEO"),
        Microsoft=>println!("我就是比尔盖茨")
    }
    match worker {
        Google=>println!("I am Google Company CEO"),
        Microsoft=>println!("我就是比尔盖茨")
    }
}
