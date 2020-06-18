fn main() {
    let arg_array = arg_to_array(&arg);
    println!("arg result==?,{:?}", arg_array);
}



// 正确的做法
fn arg_to_array(arg: &String) -> Vec<&str> {
    println!("入参=>{}", arg);
    let mut vec_array = vec![];
    let temp_arg=arg;
    let split_arg = temp_arg.split(" "); // TODO need support more blank space
    for cmd_compose in split_arg {
        if cmd_compose.len() > 0 {
            vec_array.push(cmd_compose)
        }
    };
    return vec_array;
}

// 错误的做法

/*
fn main() {
    let arg_array = arg_to_array(arg);
    println!("arg result==?,{:?}", arg_array);
}

fn arg_to_array(arg: String) -> Vec<str> {
    println!("入参=>{}", arg);
    let mut vec_array = vec![];
    let temp_arg=arg;
    let split_arg = temp_arg.split(" "); // TODO need support more blank space
    for cmd_compose in split_arg {
        if cmd_compose.len() > 0 {
            vec_array.push(cmd_compose)
        }
    };
    return vec_array;
}

*/