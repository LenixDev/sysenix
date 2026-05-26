pub fn parse_response(raw: &str) -> String {
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