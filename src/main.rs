use std::{
    fs::{self, File},
    io::{self, ErrorKind, Write},
};

use serde_json::{json, Value};

fn main() {

    let file_name = ".counting_record";

    let record_content = fs::read_to_string(file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            println!("请输入一个阈值，超出该值后会进行警告");
            let mut count_max = String::new();

            io::stdin()
                .read_line(&mut count_max)
                .expect("Failed to read line");

            let count_max: u32 = match count_max.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
            };

            let record = json!({
                "count_max": count_max,
                "input_value": 0
            });
            // 进行创建文件写入值
            let mut record_file = File::create(file_name).expect("create failed");
            record_file
                .write_all(record.to_string().as_bytes())
                .expect("write failed");
            return record.to_string();
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let v: Value = serde_json::from_str(&record_content).unwrap();

    println!("您设置的阈值为：{}", v["count_max"]);

    let mut count_current = v["input_value"].as_i64().unwrap();


    println!("当前的初始值为{}", count_current);

    loop {
        println!("请输入累加的值:");
        let mut input_value = String::new();
        io::stdin()
            .read_line(&mut input_value)
            .expect("Failed to read line");

        if input_value.contains("flush") {
            fs::remove_file(file_name).expect("could not remove file");
            println!("清空数据，请重启任务");
            break;
        }

        let input_value: i64 = match input_value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        count_current += input_value;

        println!("当前的值为:{}", count_current);

        let record = json!({
            "count_max": v["count_max"].as_u64(),
            "input_value": count_current
        });

        let mut record_file = File::create(file_name).expect("create failed");
        record_file
            .write_all(record.to_string().as_bytes())
            .expect("write failed");

        let value_pass = count_current - v["count_max"].as_i64().unwrap();

        if value_pass > 0 {
            println!("警告，超过阈值！");
            println!("警告，超过阈值！");
            println!("警告，超过阈值！");
            println!("警告，超过阈值！");
            println!("警告，超过阈值！");
            println!("警告，超过阈值！");
            println!("警告，超过阈值！");
            println!("超出的值为：{}", value_pass);
        }
    }
}
