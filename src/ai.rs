use std::io::{Read, Write};
use std::net::TcpStream;

pub fn ask(prompt: &str) -> String {
  let body = format!(
    r#"{{"model":"llava","prompt":"{}","stream":false}}"#,
    prompt
  );

  let request = format!(
    "POST /api/generate HTTP/1.0\r\nHost: 127.0.0.1\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
    body.len(),
    body
  );

  let mut stream = TcpStream::connect("127.0.0.1:11434").unwrap();
  stream.write_all(request.as_bytes()).unwrap();

  let mut response = String::new();
  stream.read_to_string(&mut response).unwrap();

  response
}