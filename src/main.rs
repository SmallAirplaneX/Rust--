use std::io; //预导入

fn main() {//声明函数体
    println!("猜数字!"); //一个宏

    let mut guess = String ::new(); //let 声明的变量不可变，添加mut关键字使其可变

    io::stdin().read_line(&mut guess).expect("无法读取");//&解释为引用,当返回的值为err时，将调用expect方法

    println!("你猜测的数字为，{}",guess);//{}，占位符

    //rust
}
