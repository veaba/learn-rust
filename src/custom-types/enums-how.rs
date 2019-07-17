#[derive(Debug)]//去掉时能自动为Student实现std::fmt::Debug特性。
enum Employee {
    X=11
}

fn main() {
    let x = Employee::X;
    match x {
        Employee::X=>{ println!("{:?}",x)}
    }
    let x = Employee::X;
    match x {
        Employee::X=>{ println!("{:?}",x)}
    }

    println!("oo-{:?}",Employee::X);
    println!("{}", Employee::X as i32);//打印出来11

}

