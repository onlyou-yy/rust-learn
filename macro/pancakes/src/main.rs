use hello_macro::HelloMacro; // 将 trait HelloMacro 搬到当前文件，因为 hello_macro_derive 要为指定结构体实现它 
use hello_macro_derive::HelloMacro;// 这是 derive 的名字

// 这里的 HelloMacro 是 hello_macro_derive::HelloMacro
#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
    println!("Hello, world!");
}
