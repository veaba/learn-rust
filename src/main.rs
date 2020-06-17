// struct Info {
//     s: String,
// }

// fn fn_a(info: Info) {
//     // println!("in fn_a,{:?}", info.s);
//     println!("in fn_a");
// }

// fn main() {
//     let foo = Info {
//         s: "abc".to_string(),
//     };
//     fn_a(foo);
//     fn_a(foo);
// }

struct Style {
    width: u32,
    height: u32,
}

impl Style {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Style {
        width: 30,
        height: 90,
    };
    println!("{:?}", rect.area())
}
