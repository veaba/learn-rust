/**
@desc 类似 C 中的 switch
@通过关键字match提供模式匹配
*/

fn main(){
	let number=1;
    println!("the number:{}",number);
    // prime
    match number {
        1=>println!("One"),
        2=>println!("Two"),
        3|4|5|6|7|11=>println!("匹配的数字"),
        12...19=>println!("A ten哇"),
        // 不是特殊的
        _=>println!("default"),
    }
    let boolean =true;
    let binary =match boolean
        {
            false=>0,
            true=>1
        };
    println!("{}-{}",boolean,binary);
}
