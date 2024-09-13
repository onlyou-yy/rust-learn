use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

enum PoolCreationError {
    SizeErr(String),
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread: thread::JoinHandle<()> = thread::spawn(move || loop {
            // 调用 recv 会阻塞当前线程，所以如果还没有任务，其会等待直到有可用的任务
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worder {id} got a job;executing");

            job();
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
        Worker { id, thread }
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
        self.sender.send(job).unwrap();
    }
}
