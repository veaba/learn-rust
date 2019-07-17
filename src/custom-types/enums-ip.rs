#[derive(Debug)]
enum IP{
    V4(u8,u8,u8,u8),
    V6(String)
}
fn main(){
    let ipv4= IP::V4(127,0,0,1);
    let ipv6 =IP::V6(String::from("::1"));
    println!("ipv4：{:?}",ipv4);
    println!("ipv6：{:?}",ipv6);
}
