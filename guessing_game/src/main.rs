use std::{collections::HashMap, fs::File, string};

use crate::garden::vegetables::asparagus::add_two;

mod garden;

/**
 * 给定一系列数字，使用 vector 并返回这个列表的中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助）。
 */
fn calculate_median_and_mode(numbers: Vec<i32>){
    let len: usize = numbers.len();
    
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort_unstable();

    let median = if len % 2 == 1 {
        Some(sorted_numbers[len / 2])
    }else{
        Some((sorted_numbers[len / 2 - 1] + sorted_numbers[len / 2]) / 2)
    };

    println!("the median number is:{}",median.unwrap());

    let mut count_map = HashMap::new();
    for &number in &sorted_numbers {
        *count_map.entry(number).or_insert(0) += 1;
    }

    let mode = count_map.iter().max_by_key(|&(_,&count)| count).map(|(&num,_)| num);
    println!("the mode is:{}",mode.unwrap());
}

fn main() {
    // let numbers = vec![1,4,5,2,3,2,5,6,3,8,5];
    // calculate_median_and_mode(numbers);

    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {error:?}"),
    // };
    let num1 = 32;
    let num2 = 12;
    let num3 = num1 + num2;
    println!("app");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(&string1, string2);
    println!("The longest string is {result}");
}

fn longest<'a>(x:&'a str,y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }else {
        y
    }
}

