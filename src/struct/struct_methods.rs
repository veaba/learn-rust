// 结构体方法

// 结构体定义
#[derive(Debug)]
struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32,
}

// imple 定义方法，结构体的方法是必须要impl 来定义的！

impl Site {
    fn get_name(&self) -> String {
        String::from(&self.name) + "===> 获取名字"
        // self.name + String::from(": This is my name!")
    }
}

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

    let x = site.get_name();
    println!("xxx=>{}", x);
}
