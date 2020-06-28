#[derive(Debug)]
struct User {
    username: String,
    email:String,
    age:u64,
    active:bool
}



fn main() {
    let user_a=User{
        email:String::from("abc@qq.com"),
        username:String::from("abc"),
        age:26,
        active:true,
    };
    
    let user_b=User{
        email:String::from("cdf@qq.com"),
        username:String::from("cdf"),
        ..user_a
    };
    println!("user a==>{:#?}",user_a);
    println!("user b==>{:#?}",user_b);
}