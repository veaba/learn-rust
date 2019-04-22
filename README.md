# learn-rust  

## 疑问
- 如何if == string？

- while循环和for循环的区别：
> 如果数组的索引长度在运行时增加，那么while循环显示错误，但在for循环的情况下不会发生这种情况。 因此，可以说for循环增加了代码的安全性并消除了错误的可能性。

- 所有权 
 - 代码块拥有资源时，被称为所有权。
 - 代码块创建一个包含资源的对象。
 - 当控件到达末尾时，对象将被销毁，资源将被释放
  

## 内存和分配

![堆栈]("./static/images/stack-heap.png");

- 堆栈存储器
	- 循环放置
	- 相反顺序移除
	- 遵从“后进先出”，进电梯 原则
	- 始终 首先删除最后插入的数据
	- `堆栈内存`有组织的内存，比`堆内存`更快
	- 访问内存的方式
	- 编译时数据大小未知，则`堆内存`用于存储内容

- 堆内存
	- `堆内存`是有组织的内存	
	- os 在`堆内存`中找到一个空的空格并返回一个指针，这叫 `在对上分配`

![内存图片]("./static/images/memory.png");

- 第一步
	- 向量v1与值 1 2 3 '666'绑定，四个部分组成
	- 指向存储器中指向存储在内存的数据的指针 长度和向量的容量
	- 这部分存储在堆栈中，二数据存储在堆内存中

![one]("./static/images/one.png");

- 第二步
	- `v1` 向量分配给向量`v2`
	- 指针，长度、容量将复制到堆栈中
	- 但不会讲数据复制到堆内存中 
	- 
	- `v1`、`v2` 都超出范围时，两者都会尝试释放内存
	- 这会导致双重空闲内存，从而使得内存损坏~
	
![two]("./static/images/two.png");

- 第三步
	- rust 避免了第二步的内存问题
	- rust没有复制分配的内存，则认为`v1`向量不再有效
	- 当 `v1`超出范围时，它不需要释放`v1`的内存
	
![three]("./static/images/three.png");

```rust
fn main(){
	let v1= vec![1,2,3,"666"];
    let v2 = v2;
}

```

### 特征复制
- 复制特征是特殊的注释
- 放在存储堆栈上的整数类型上
- 如果类型使用了复制特征，则复制之后还可以使用旧变量
- 复制类型
	- 所有整数类型，如 `u32`
	- 布尔类型 `bool`:`true`, `false`
	- 所有浮动类型，如`f64`
	- 字符类型，如 `char`


### 所有权和函数

## 常识

|name|desc|
|---|---|
|.rs|rust文件后缀|
|Cargo.lock||
|Cargo.toml||
|xxx.iml||
|src目录|源码|
|target目录||
|mnt i| i是可以更改变量，可变变量|
|双引号|给字符/对象|

## install
### version
> rustc --version
### update 更新
> rustup update
### uninstall 下载
> rustup self uninstall
## rustup

## cargo

## 第一个程序

```rust
fn main(){
    print!("Hello,world!");
}
```


执行命令
> rustc hello.rs 

会生成` hello.exe`、`hello.pdb`，windows 下运行hello.exe才会运行

## 函数

### print!()  
- print!(双引号)，且只能是字符串

```rust
fn main(){
    let a=9;
    if  a==9 {
        print!("aaa");
    }
}
```

### println!()

## if判断语句
```rust
fn main(){
    let a=9;
    if  a==8 {
        println!("aaa");
    }
    else if a>8 {
        println!("bbb");
    }
    else{
        println!("ccc");
    }
}
```
## if in a let语句

## error


> ^^^ expected integer, found char

## 循环
### loop 循环
- 需要mnt 声明变量，
- 无法使用i++ 自增加1，而是i+=1
- break 跳出loop
```rust
fn main(){
	let mut i =1;
    loop {
        println!("{}",i);
        if i==10{
            break
        }
        i+=1
    }
}

```
### for循环

- 循环数组
```rust
//TODO 循环数组
fn main(){
	let res =["mango","apple","banana","litchi","watermelon"];
    for i in res.iter() { //iter()方法
            println!("{}",i)
    }
}
```

- 循环对象
```rust
//TODO 循环对象
```
### while循环
- 索引不正确，循环有问题
- 每次迭代前进行条件检查，速度慢
- 如果外部引发条件变化，则可能会引发死循环的异常
```rust
fn main(){
	let mut i=0;
    while i<=10 {
        println!("{}",i);
        i+=1
    }
}
```
## 数组

### 访问方式 和js 一样 arr[1]

## 方法
### .iter()
- 访问数组的每个元素

## 所有权

- Rust中，每个值都有一个与之关联的变量，并成为其所有者
- 一次只能有一个所有者
- 当所有者超出范围是，与其关联的值将被销毁

> a被回收，编译直接报错
```rust
// 
fn main(){
	let a =20;
    let b = a;
    println!("{}",a)
}
```

>但是 下面这样就不会被回收~~ TODO ？why？
```rust
fn main1(){
	let a =20;
    let b = a;
    println!("{}",b);
    println!("{}",a)
}
```

