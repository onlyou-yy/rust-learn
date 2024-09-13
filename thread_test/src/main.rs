use std::{sync::{mpsc, Arc, Mutex}, thread, time::Duration};

fn main() {
    let handler = thread::spawn(|| {
        for i in 1..30 {
            println!("spawn number is {i}");
        }
    });

    for i in 1..2 {
        println!("main number is {i}");
    }
    handler.join().unwrap();

    let list = vec![1,2,3];
    let handler2 = thread::spawn(|| {
        println!("list is {list:?}");
        for item in list {
            println!("list item is {item}");
        }
    });

    handler2.join().unwrap();


    let (tx,rx) = mpsc::channel();
    thread::spawn(move || {
        // let message = String::from("hello");
        // tx.send(message).unwrap();
        let list =vec!["hello","world","huixing","giegie"];
        for item in list {
            tx.send(item).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // let result = rx.recv().unwrap();
    // println!("message: {result}");
    for receive in rx {
        println!("message: {receive}");
    }

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");

    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for i in 1..10{
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(handler);
    }

    for hand in handlers {
        hand.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    let screen: Screen<Button> = Screen {
        components:vec![
            Box::new(Button {
                width:10,
                height:10,
            })
        ]
    };
    let but = &screen.components[0];
    but.draw();
}

pub trait Draw {
    fn draw(&self);
}
// pub struct Screen {
//     pub components: Vec<Box<dyn Draw>>,
// }

pub struct Screen<T: Draw> {
    pub components: Vec<Box<T>>,
}

pub struct Button{
    width:i32,
    height:i32
}

impl Draw for Button{
    fn draw(&self) {
        println!("print");
    }
}