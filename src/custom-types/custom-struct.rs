/**
@desc rust 通过自定义类型主要通过以下两个关键形成：

- struct 声明一个结构
- enum   声明一个枚举

@desc 常量可以通过`const` `static`  关键字 声明

@desc 结构
    - 元组结构，基本上是命名为元组的。
    - 经典的C结构
    - 无字段的单元结构对于泛型很有用
*/
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    //这是啥子？
    age: u8,
}

// 一个单位结构
struct Nil;//哈？

// 一个元组结构
struct Pair(i32, f32);

// 两个字段的结构
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    let name = "Jogel";
    let age = 27;
    let jogel = Person { name, age };

    println!("{:?}", jogel); //Person {name:"jogel",age:27} 问题，如何移除这个 “Person”

    let point: Point = Point { x: 3.14, y: 99.666 };

    println!("第一组坐标：({},{})", point.x, point.y);//坐标：(3.14,99.666)

    // Make a new point by using struct update syntax to use the fields of our other one
    let new_point = Point { x: 0.6, ..point };//？展开point，使用展开符，存在，则使用前面，前面使用替换后面的？？
    println!("第二组坐标：({},{})", new_point.x, new_point.y);//(0.6,99.666)

    // 测试展开符号的入参方式

    let old_point = Point {y: 2.333,..new_point};
    println!("第三组坐标：({},{})", old_point.x, old_point.y);//(0.6,2.333)

    // Destructure the point using a `let` binding

    let Point {x:m_x,y:m_y}=point;
    println!("({},{})",m_x,m_y);

    let _rectangle= Rectangle{
        p1:Point{x:m_y,y:m_x},
        p2:point
    };

    let _nil=Nil;

    // implemented 实现 执行？

    // println!("_nil:{:?}",_nil); TODO

    let pair =Pair(99,99.99);

    println!("pair 包含 ({:?},{:?})",pair.0,pair.1);

    // 解构一个元组结构 Destructure a tuple struct decimal 小数点
    let Pair(integer,decimal)=pair;
    println!("pair 包含 ({:?},{:?})",integer,decimal);
}

