// 结构体引用
// 结构体定义
#[derive(Debug)]
struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32,
}

//

fn main() {
    let domain = String::from("domain.com");
    let name = String::from("Hello world!");
    let google = Site {
        domain,
        name,
        nation: String::from("Han"),
        found: 6,
    };

    println!("==> {:#?}", google);

    let site = Site {
        domain: String::from("baidu.com"),
        name: String::from("百度一下，你就知道"),
        ..google
    };
    println!("\n{:#?}", site);
}
