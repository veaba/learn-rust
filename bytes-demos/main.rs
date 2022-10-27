use bytes::{BufMut, BytesMut};

fn main() {
    println!("开始oooooooooooooooooooo===>");
    let mut buf = BytesMut::with_capacity(1024);
    buf.put(&b"hello world"[..]);
    buf.put_u16(1234);
    println!("{:?}", buf);
    println!("buf len{}", buf.len());

    println!("结束oooooooooooooooooooo===>");
}
