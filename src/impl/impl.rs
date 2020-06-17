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
