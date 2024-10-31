use async_std::net::TcpListener;
use async_web_server::handle_connection;
use futures::StreamExt;

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("server running in http://127.0.0.1:3000");
    
    listener.incoming().for_each_concurrent(/* limit */ None,|tcpstream| async move {
        let tcpstream = tcpstream.unwrap();
        handle_connection(tcpstream).await;
    }).await;

}