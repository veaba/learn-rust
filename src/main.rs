use bytes::{BufMut, Bytes, BytesMut};

fn main() {
    println!("开始oooooooooooooooooooo===>");
    let mut buf = BytesMut::with_capacity(1024);
    buf.put(&b"abcd"[..]);
    // buf.put(&b"hello world"[..]);
    // buf.put_u16(1234);
    println!("{:?}", buf);
    println!("buf len=>{}", buf.len());

    println!("结束oooooooooooooooooooo===>");

    let b = Bytes::new();
    println!("{:?}", &b[..]);
    println!("{:?}", b);

    let c = String::from("abcdefg");

    println!("{:?}", &c[..]);
    println!("{}", c);
}
