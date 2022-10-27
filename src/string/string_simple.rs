// TODO 如何简写一下代码？
fn main() {
    let a = String::from("aa");
    let b = String::from("bb");
    // let c: String = a | b;
    let c;
    if a.len() > 0 {
        c = a
    } else {
        c = b
    }

    println!("{}", c);
}
