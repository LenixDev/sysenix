use std::io::{Read, Write};
use std::net::TcpStream;

fn parse_response(raw: &str) -> String {
  // find the HTTP body — skip headers
  let body = if let Some(i) = raw.find("\r\n\r\n") {
    &raw[i + 4..]
  } else {
    raw
  };

  // extract "response":"..." value
  if let Some(start) = body.find("\"response\":\"") {
    let rest = &body[start + 12..];
    if let Some(end) = rest.find("\"") {
      return rest[..end].to_string();
    }
  }

  body.to_string()
}

pub fn ask(prompt: &str, image_base64: &str) -> String {
  let body = format!(
    r#"{{"model":"llava","prompt":"{}","images":["{}"],"stream":false}}"#,
    prompt, image_base64
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