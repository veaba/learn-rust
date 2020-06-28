fn main() {
    let a = "aa-a";
    let b = "bb";
    let c;
    if a.len() > 0 {
        c = a
    } else {
        c = b
    }
    println!("{}", c);
}
