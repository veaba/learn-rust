#[derive(Debug)]
// 结构
struct Triangle {
    base: f64,
    height: f64,
}

// 声明一个形状，叫HasArea
// 这里对包含的area函数做了声明
trait HasArea {
    fn area(&self) -> f64;
}

// 啥意思这里？
// HasArea 是在Triangle类型上实现的
impl HasArea for Triangle {
    fn area(&self) -> f64 {
        0.5 * (self.base * self.height)
    }
}

fn main() {
    let a = Triangle {
        base: 10.5,
        height: 17.4,
    };
    let triangle_area = a.area();
    println!("==>{}", triangle_area);
    println!("==>{:#?}", a);