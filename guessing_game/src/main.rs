// use std::{cmp::Ordering, io};

// use rand::Rng;


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    // let tem_str: &str;
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            // tem_str = &s[0..i];
            // return tem_str.to_string();
            return &s[0..i];
        }
    }
    // tem_str = &s[..];
    // return tem_str.to_string();
    return &s[..];
}

# [derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32,
}

# [derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}

fn main() {

    let mut s = String::from("hello world");

    let str: String = first_word(&s).to_string();

    // clone引用得到的依然是引用，不会创建新的字符串
    // let str: &str = first_word(&s).clone();
    s.clear();
    println!("{}",str);

    let s1 = "sfaf";
    let s2 = s1.to_string();
    println!("{},{}",s1,s2);

    let tt = (1,2,3);
    println!("{}",tt.1);

    let mut rect: Rectangle = Rectangle {
        width:20,
        height:30,
    };
    println!("{:#?}",rect);
    rect.width = 30;
    println!("{:#?}",rect);

    
    let home = IpAddr::V4(String::from("127.0.0.1"));
    println!("{home:#?}");

    struct IpStruct {
        ip:IpAddr,
        address:String
    }
    let ips = IpStruct {
        ip:IpAddr::V4(String::from("hhhh")),
        address:String::from("sdpfasf")
    };
    
    let dice_roll = 9;
    match dice_roll {
        3 => 2,
        7 => 1,
        other => other
    };

    let x = Some(1);
    match x {
        Some(i) => Some(i + 1),
        None => None
    };


    // let mut s = String::from("hello");

    // let r1 = &s; // 没问题
    // let r2 = &s; // 没问题
    // println!("{} and {}", r1, r2);
    
    // let r3 = &mut s;
    // println!("{}", r3);

    // println!("输入值{}",b' ');

    // let secret_number: u32 = rand::thread_rng().gen_range(1..=5);
    // println!("随机数为：{secret_number}");

    // loop {
    //     let mut guess: String = String::new();
    //     println!("请输入你的猜测：");
    //     io::stdin().read_line(&mut guess).expect("出错啦");

    //     let guess: u32 = match guess.trim().parse() {
    //         Err(_) => continue,
    //         Ok(num) => num
    //     } ;
    //     println!("你输入的是：{}", guess);
    //     match guess.cmp(&secret_number) {
    //         Ordering::Greater => {
    //             println!("太大了");
    //             println!("--------------");
    //         },
    //         Ordering::Less => {
    //             println!("太小了");
    //             println!("--------------");
    //         },
    //         Ordering::Equal => {
    //             println!("猜对了");
    //             break;
    //         },
    //     }
    // }
    
}
