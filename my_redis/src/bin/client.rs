use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

#[tokio::main]
async fn main(){
  let (tx,mut rx) = mpsc::channel(32);  
  let tx2 = tx.clone();

  //  将消息通道接收者 rx 的所有权转移到管理任务中 
  let manager = tokio::spawn(async move {
    let mut  client = client::connect("127.0.0.1:6379").await.unwrap();
    while let Some(cmd) = rx.recv().await {
       match cmd {
          Command::Get { key ,resp} => {
            let ret = client.get(&key).await;
            let _ = resp.send(ret);
            println!("Get result");
          },
          Command::Set { key, val ,resp} => {
            let ret = client.set(&key, val).await;
            if ret.is_ok() {
              let _ = resp.send(Ok(Some(Bytes::from_static(b"Ok"))));
            }
            println!("Get result: {ret:?}");
          }
       } 
    }
  });

  let t1 = tokio::spawn(async move {
    let (resp_tx,resp_rx) = oneshot::channel();
    let cmd = Command::Get { key: "hello".to_string(),resp:resp_tx };
    tx.send(cmd).await.unwrap();
  });

  let t2 = tokio::spawn(async move {
    let (resp_tx,resp_rx) = oneshot::channel();
    let cmd = Command::Set { key: "hello".to_string(), val: "bar".into(),resp:resp_tx };
    tx2.send(cmd).await.unwrap();
  });
  
  println!("t1 run start");
  t1.await.unwrap();
  println!("t2 run start");
  t2.await.unwrap();
  println!("manager run start");
  manager.await.unwrap();
}

#[derive(Debug)]
enum Command {
  Get{
    key:String,
    resp:Responder<Option<Bytes>>
  },
  Set{
    key:String,
    val:Bytes,
    resp:Responder<Option<Bytes>>
  }
}

/// 管理任务可以使用该发送端将命令执行的结果传回给发出命令的任务
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;