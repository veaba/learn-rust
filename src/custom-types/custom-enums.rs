use std::borrow::ToOwned;

/**
@desc enums 枚举

*/

enum WebEvent{
    // 一个枚举可以是单位
    PageLoad,
    PageUnload,
    // 类似元组
    Keypress(char),
    Paste(String),
    // 或者结构struct
    Click{x:i64,y:i64}
}

// A function which takes a `WebEvent` enum as an argument and
// 枚举webEvent 作为函数的参数,可以理解为js中的对象里面的方法
fn inspect(event:WebEvent){
    // match干嘛的？
    match event {
        WebEvent::PageLoad=>println!("page load"),
        WebEvent::PageUnload=>println!("page unload"),
        WebEvent::Keypress(c)=>println!("pressed '{}'.",c),
        WebEvent::Paste(s)=>println!("pasted\"{}\".",s),
        WebEvent::Click {x,y}=>println!("click at x={},y={}",x,y)
    }
}

fn main(){
    let load = WebEvent::PageLoad;
    let unload =WebEvent::PageUnload;
	let pressed = WebEvent::Keypress('x');
    let pasted = WebEvent::Paste("wa.. My love".to_owned());//干嘛？
    let click= WebEvent::Click {x:20,y:999};
    inspect(load);
    inspect(unload);
    inspect(pressed);
    inspect(pasted);
    inspect(click);
}
