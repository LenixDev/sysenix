use std::process::Command;

pub fn ask(prompt: &str, image_base64: &str) -> String {
  let api_key = std::env::var("KEY").expect("KEY not set");

	let safe_prompt = prompt
  .replace('\\', "\\\\")
  .replace('"', "\\\"")
  .replace('\n', "\\n")
  .replace('\r', "");

  let body = format!(
    r#"{{"model":"meta-llama/llama-4-scout-17b-16e-instruct","messages":[{{"role":"user","content":[{{"type":"text","text":"{}"}},{{"type":"image_url","image_url":{{"url":"data:image/png;base64,{}"}}}}]}}],"max_tokens":1024}}"#,
    safe_prompt, image_base64
  );

  let tmp = "/tmp/sysenix_request.json";
  std::fs::write(tmp, &body).unwrap();

  let output = Command::new("curl")
    .args([
      "-s",
      "-X", "POST",
      "https://api.groq.com/openai/v1/chat/completions",
      "-H", "Content-Type: application/json",
      "-H", &format!("Authorization: Bearer {}", api_key),
      "-d", &format!("@{}", tmp),
    ])
    .output()
    .unwrap();

  let raw = String::from_utf8(output.stdout).unwrap();
  parse_response(&raw)
}


pub fn parse_response(raw: &str) -> String {
  let json: serde_json::Value = serde_json::from_str(raw).unwrap_or_default();
  json["choices"][0]["message"]["content"]
    .as_str()
    .unwrap_or("")
    .to_string()
}