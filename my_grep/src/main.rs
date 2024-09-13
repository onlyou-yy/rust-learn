use std::{env,process};

use my_grep::Config;

fn main() {
    // let args:Vec<String> = env::args().collect();
    // println("{args:?}"); // 获取引用
    // dbg!(args);//获取所有权
    

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("config build error:{}",err);
        process::exit(1);
    });

    if let Err(e) = my_grep::run(config){
        eprintln!("读取文件内容错误:{e}");
        process::exit(2);
    }
}