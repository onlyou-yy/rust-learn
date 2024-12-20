use std::{fs, time::Duration};
use async_std::prelude::*;

pub async fn handle_connection(mut stream:impl async_std::io::Read + async_std::io::Write + std::marker::Unpin) {
  let mut buffer = [0;1024];
  stream.read(&mut buffer).await.unwrap();

  let get = b"GET / HTTP/1.1\r\n";
  let sleep = b"GET /sleep HTTP/1.1\r\n";

  let (status_line,filename) = if buffer.starts_with(get) {
      println!("incoming /");
      ("HTTP/1.1 200 OK\r\n\r\n","hello.html")
  } else if buffer.starts_with(sleep) {
      println!("incoming /sleep");
      // 不能用这种方式模拟慢请求，该函数是阻塞的，它会让当前线程陷入睡眠中，导致其它任务无法继续运行！
      // thread::sleep(Duration::from_secs(5));
      async_std::task::sleep(Duration::from_secs(5)).await;
      ("HTTP/1.1 200 OK\r\n\r\n","hello.html")       
  } else {
      println!("incoming 404");
      ("HTTP/1.1 404 NOT FOUNd\r\n\r\n","404.html")
  };

  let content = fs::read_to_string(filename).unwrap();
  
  let response = format!("{status_line}{content}");
  stream.write_all(response.as_bytes()).await.unwrap();
  stream.flush().await.unwrap();
}