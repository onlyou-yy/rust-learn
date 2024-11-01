use tokio::{io::{self, AsyncReadExt, AsyncWriteExt}, net::TcpListener};

#[tokio::main]
async fn main() -> io::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:6142").await?;
  let (socket,_) = listener.accept().await?;
  // 任何一个读写器( reader + writer )都可以使用 io::split 方法进行分离，最终返回一个读取器和写入器，这两者可以独自的使用
  let (mut rd, mut wr) = io::split(socket);

  // 创建异步任务，在后台写入数据
  tokio::spawn(async move {
    wr.write_all(b"hello\r\n").await?;
    wr.write_all(b"world\r\n").await?;

    // 有时，我们需要给予 Rust 一些类型暗示，它才能正确的推导出类型
    Ok::<_, io::Error>(())
  });

  let mut buf = vec![0; 128];

  loop {
    let n = rd.read(&mut buf).await?;

    if n == 0 {
        break;
    }

    println!("GOT {:?}", &buf[..n].utf8_chunks());
  }

  Ok(())
}