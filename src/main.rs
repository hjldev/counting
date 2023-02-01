use std::{io, cmp::Ordering};

fn main() {
    println!("请输入一个阈值，超出该值后会进行警告");
    let mut count_max = String::new();
    io::stdin()
        .read_line(&mut count_max)
        .expect("Failed to read line");

    let count_max: u32 = match count_max.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("您设置的阈值为：{}", count_max);

    let mut count_current = 0;

    loop {
        println!("请输入累加的值:");
        let mut input_value = String::new();
        io::stdin()
        .read_line(&mut input_value)
        .expect("Failed to read line");
        let input_value: u32 = match input_value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        count_current += input_value;

        println!("当前的值为:{}", count_current);
        if count_current > count_max {
            println!("警告，超过阈值！");
            println!("警告，超过阈值！");
            println!("警告，超过阈值！");
            println!("警告，超过阈值！");
            println!("警告，超过阈值！");
            println!("警告，超过阈值！");
            println!("警告，超过阈值！");
            panic!("超过阈值，程序结束");
        }
    }
}
