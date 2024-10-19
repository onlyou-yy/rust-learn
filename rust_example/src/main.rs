use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        println!("thread start run");
        thread::sleep(Duration::from_secs(5));
        println!("thread end run");
    });

    handle.join().unwrap();
    println!("end");
}
