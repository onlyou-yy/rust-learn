use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub enum PoolCreationError {
    SizeErr(String),
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub enum Message {
    NewJob(Job),
    Terminate,
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread: thread::JoinHandle<()> = thread::spawn(move || {
            loop {
                // 调用 recv 会阻塞当前线程，所以如果还没有任务，其会等待直到有可用的任务
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {id} got a job;executing");
                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });
        // 不能像下面这样写
        // 因为while let（if let 和 match）直到相关的代码块结束都不会丢弃临时值,
        // 也就是在执行 job() 时其他worker还是不能访问 receiver 的，因为还没解锁
        // 当 let 语句结束时任何表达式中等号右侧使用的临时值都会立即被丢弃，即对互斥器解锁，可以执行其他的worker
        // let thread = thread::spawn(move || {
        //     while let Ok(job) = receiver.lock().unwrap().recv() {
        //         println!("Worker {id} got a job; executing.");

        //         job();
        //     }
        // });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl Drop for ThreadPool {
    // 当程序出错时会运行这里
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        /**
         * 先给每个消费者发送一个停机消息，
         * 因为只有一个消费者，所以当一个消息被worker被接收后，其他的worker并不能接受到消息，
         * 下面两个循环不写在一起的原因是，如果写到一起，此时如果worker0正在处理请求，消息过来了，worker0就不能接受到信息
         * 而被其他worker收到终止消息并停止。我们会一直等待第一个 worker 结束，不过它永远也不会结束因为第二个线程接收了终止消息。死锁！
         */
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl ThreadPool {
    /// 创建线程池
    /// 线程池中的线程数量
    ///
    /// # Panics
    /// `new` 函数在 size 为 0 时会 panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Err(PoolCreationError::SizeErr(String::from("线程数必须大于0")));
        }
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
