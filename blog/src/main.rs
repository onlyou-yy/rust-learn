use blog::Post;

use std::fmt;
use std::ops::{Deref,DerefMut};

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Wrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("",post.content());

    post.request_review();
    assert_eq!("",post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today",post.content());

    let mut w = Wrapper(vec![String::from("hello"), String::from("world")]);

    // 直接访问内部 Vec<String> 的方法
    w.push(String::from("rust"));
    println!("w = {w}");

    // 使用迭代器
    for s in w.iter() {
        println!("{}", s);
    }
}
