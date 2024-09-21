use my_proc_macro::sql;

#[my_proc_macro::my_first_attr_proc_macro("fffff")]
fn add(a: i32, b: i32) {
    println!("{a} + {b} = {}", a + b);
}

fn main() {
    sql!(select * from table1 where id = 100 and count = 1 order by time desc);

    hello();
}
