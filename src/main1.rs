// use std::{str::Bytes, fmt::format};

// use regex::bytes;

// fn main() {
//     // let a = "config=config.toml";
//     let b = "config=config.toml port=80 start stop";

//     let c=b.split(" ");
//     let mut vec_array= vec![];
//     for x in c{
//         println!("==>{}",x);
//         vec_array.push(x);
//     }
//     println!("vec_array==>{:?}",vec_array); // ["config=config.toml", "port=80", "start", "stop"]
// }

// fn main() {
//     println!("=>{:?}", 0x7c0 >> 2)
// }

// fn main() {
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);

//     assert_eq!(vec.len(), 2);

//     assert_eq!(vec[0], 1);

//     assert_eq!(vec.pop(), Some(2));
//     println!("vec==>{:?}", vec);
//     // assert_eq!(vec.len(), 1);

//     // vec[0] = 7;
//     // assert_eq!(vec[0], 7);

//     // vec.extend([1, 2, 3].iter().copied());

//     // for x in &vec {
//     //     println!("{x}");
//     // }
//     // assert_eq!(vec, [7, 1, 2, 3]);
// }

fn main() {
  // let b = [11, 22];

  // let mut v = vec![1, 2, 3, 4];
  // let new = [7, 8, 9];
  // let u: Vec<_> = v.splice(1..3, new).collect();
  // println!("vec==>{:?}", u);
  // println!("dd={:?}", v.splice(1..3, new))

  // let mut vec = vec![1, 2, 3];
  // let mut vec2 = vec![4, 5, 6];
  // vec.append(&mut vec2);

  // let mut vec = Vec::with_capacity(10);
  // vec.extend([1, 2, 3]);
  // assert_eq!(vec.capacity(), 10);
  // vec.shrink_to(4);
  // let mut vec = vec![1];
  // vec.extend_from_slice(&[2, 3, 4]);
  // assert_eq!(vec, [1, 2, 3, 4]);
  // a.rotate_left(1);
  // println!("dd={:?}", a);
  // let b = a[1..5];
  // let mut vec1 = vec![133, 22];
  // println!("cc={:?}", vec);
  // let vec2 = vec1.extend([11]);
  // println!("vec1={:?}", vec1);
  // println!("vec2={:?}", vec2);
  // let mut vec3 = vec![11];
  // let xx = [22, 33];
  // vec3.extend(&xx);
  // println!("vec3={:?}", vec3);
  // println!("xx={:?}", xx);

  // let mut vec = vec![1, 2, 3];
  // vec.insert(0, 999);

  // println!("vec3={:?}", vec);

  // let mut mem = Bytes::from("Hello world");
  // let a = mem.slice(0..5);

  // println!("vec3={:?}", a);

  // let b = mem.split_to(6);

  // println!("vec3={:?}", mem);
  // assert_eq!(b, "Hello ");

  // charCodeAt()
  // let ch  = "H".escape_unicode();
  // let ch_unicode = format!("{:X}",ch);
  // println!("={}==>",ch.next());
  // assert_eq!('\u{CA0}', 'ಠ');

  // let c = 'H';
  // let i = 'H' as u32;
  // println!("{}",i)
  // assert_eq!(128175, i);

  // let c = "HELLO";
  // // let i = 'H' as u32;
  // let o = c.chars().nth(0).unwrap() as u32;
  // // let k = Some(o);
  // println!("{:?}", o);
  // println!("{:?}", o.unwrap());

  // 获取字符串某个 索引的值
  // let u32_byte_array = 6.1 as u32;
  // println!("{u32_byte_array}",)

  // 可变 vec
  // let mut v = vec![1, 2,[33]];
  // let two = v.pop();
  // println!("{:?}",v)

  let mut array = [""; 3];
  array[0] = "foo";
  array[1] = "bar";
  array[2] = "baz";
  println!("{:?}", array)
}
