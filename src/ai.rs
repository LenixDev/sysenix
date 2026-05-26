use std::process::Command;

pub fn ask(prompt: &str, image_base64: &str) -> String {
  let api_key = std::env::var("KEY").expect("KEY not set");

  let body = format!(
    r#"{{"contents":[{{"parts":[{{"text":"{}"}},{{"inline_data":{{"mime_type":"image/png","data":"{}"}}}}]}}]}}"#,
    prompt, image_base64
  );

  let output = Command::new("curl")
    .args([
      "-s",
      "-X", "POST",
      &format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", api_key),
      "-H", "Content-Type: application/json",
      "-d", &body,
    ])
    .output()
    .unwrap();

  let raw = String::from_utf8(output.stdout).unwrap();
  parse_response(&raw)
}

pub fn parse_response(raw: &str) -> String {
  // extract text from "parts":[{"text":"..."}]
  if let Some(start) = raw.find("\"text\":\"") {
    let rest = &raw[start + 8..];
    if let Some(end) = rest.find("\"") {
      return rest[..end].to_string();
    }
  }
  raw.to_string()
}