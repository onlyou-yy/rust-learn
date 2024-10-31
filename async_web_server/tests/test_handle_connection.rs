use std::fs;
use async_std::prelude::*;
use async_web_server::handle_connection;

struct MockTcpStream {
  read_data:Vec<u8>,
  write_data:Vec<u8>,
}

impl async_std::io::Read for MockTcpStream {
  fn poll_read(
      self: std::pin::Pin<&mut Self>,
      _: &mut std::task::Context<'_>,
      buf: &mut [u8],
  ) -> std::task::Poll<std::io::Result<usize>> {
      let size = std::cmp::min(self.read_data.len(), buf.len());
      buf[..size].copy_from_slice(&self.read_data[..size]);
      futures::task::Poll::Ready(Ok(size))
  }
}

impl async_std::io::Write for MockTcpStream {
  fn poll_write(
      mut self: std::pin::Pin<&mut Self>,
      _: &mut std::task::Context<'_>,
      buf: &[u8],
  ) -> std::task::Poll<std::io::Result<usize>> {
      self.write_data = Vec::from(buf);
      futures::task::Poll::Ready(Ok(buf.len()))
  }

  fn poll_flush(self: std::pin::Pin<&mut Self>, _: &mut std::task::Context<'_>) -> std::task::Poll<std::io::Result<()>> {
     futures::task::Poll::Ready(Ok(())) 
  }

  fn poll_close(self: std::pin::Pin<&mut Self>, _: &mut std::task::Context<'_>) -> std::task::Poll<std::io::Result<()>> {
     futures::task::Poll::Ready(Ok(())) 
  }
}

impl std::marker::Unpin for MockTcpStream {}


#[async_std::test]
async fn test_handle_connection() {
  let input_bytes = b"GET / HTTP/1.1\r\n";
  let mut content = vec![0u8;1024];
  content[..input_bytes.len()].clone_from_slice(input_bytes);

  let mut stream = MockTcpStream {
      read_data:content,
      write_data:Vec::new(),
  };

  handle_connection(&mut stream).await;
  let mut buf = [0u8;1024];
  stream.read(&mut buf).await.unwrap();

  let expected_contents = fs::read_to_string("hello.html").unwrap();
  let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{expected_contents}");

  assert!(stream.write_data.starts_with(expected_response.as_bytes()));
}