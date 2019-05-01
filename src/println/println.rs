fn main(){
	println!("{} days",365);


    // 括号里面可以使用索引
    println!("{0},this {1}.{1},啦啦 this {0},{1}","智","障");

    println!("{subject}-{verygood}-{object}",
            object="25号",
            verygood="40月",
            subject="20191年"
    );

    //{:b} 二进制
    println!("{}pf {:b} xsaojdaw,,daoud",1,2);

    //{:x} 十六进制
    println!("{}pf {:x} xsaojdaw,,daoud",1,4454);

    //对齐width个空格宽度
    println!("{number:>width$}",number=33,width=100);

    println!("{number:>width$}",number="xxxxx",width=10+3);

    // 补零。其他数字不可以
    println!("{number:>0width$}",number=1,width=10+3);
    println!("{number:<0width$}",number=1,width=10+3);

    // println!("My name is {0},{1}  {0}","world");
    println!("My name is {0}, {1} {0}","Gel","Ji");

    // 不工作，代码，TODO
    struct Structure(i32);
    println!("这个结构`{}`,不能打印...",Structure(3))
}
