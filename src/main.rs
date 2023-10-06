use std::io;

use rand::Rng; //预导入

fn main() {
    //声明函数体
    println!("猜数字!"); //一个宏

    let secret_number = rand::thread_rng().gen_range(1, 101); //生产范围内的随机数

    println!("生成的随机数为，{}", secret_number); //一个宏

    loop {
        //反复
        println!("请输入一个数字"); //一个宏

        let mut guess = String::new(); //let 声明的变量不可变，添加mut关键字使其可变

        io::stdin().read_line(&mut guess).expect("无法读取"); //&解释为引用,当返回的值为err时，将调用expect方法

        println!("你猜测的数字为，{}", guess); //{}，占位符

        //let guess: u32 = guess.trim().parse().expect("请输入数字"); //parse用于数字类型转换
        //这段代码将引起崩溃，使得程序无法重复执行，将代码修改成下面这样

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue //_,表示不关心
        };

        match guess.cmp(&secret_number) {
            //cmp()，是一个比较方法，其返回值通过match表达式进行匹配，返回值有3个类型
            //由于guess的类型为String，而secret_number是i32类型，他们是无法比较的，使用需要对guess进行定义覆盖，并设置新的类型
            std::cmp::Ordering::Less => println!("小了"),
            std::cmp::Ordering::Greater => println!("大了"),
            std::cmp::Ordering::Equal => {
                println!("对了");
                break;
            }
        };
    }
}
