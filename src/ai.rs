use std::{process::Command, sync::Mutex};

pub fn ask(
  prompt: &str,
  conversation: &Mutex<Vec<(String, String)>>,
  image_base64: &str,
) -> String {
  let api_key = std::env::var("KEY").expect("KEY not set");

  let history = conversation.lock().unwrap();
  let mut messages_json = String::new();
  for (role, content) in history.iter() {
    messages_json.push_str(&format!(
      r#"{{"role":"{}","content":"{}"}}"#,
      role,
      content.replace('"', "\\\"")
    ));
    messages_json.push(',');
  }
  messages_json.push_str(&format!(
    r#"{{"role":"user","content":[{{"type":"text","text":"{}"}},{{"type":"image_url","image_url":{{"url":"data:image/png;base64,{}"}}}}]}}"#,
    prompt.replace('"', "\\\"").replace('\n', "\\n"),
    image_base64
  ));
  drop(history);

  let body = format!(
    r#"{{"model":"meta-llama/llama-4-scout-17b-16e-instruct","messages":[{}],"max_tokens":1024}}"#,
    messages_json
  );

  let tmp = "/tmp/sysenix_request.json";
  std::fs::write(tmp, &body).unwrap();

  let output = Command::new("curl")
    .args([
      "-s",
      "-X",
      "POST",
      "https://api.groq.com/openai/v1/chat/completions",
      "-H",
      "Content-Type: application/json",
      "-H",
      &format!("Authorization: Bearer {}", api_key),
      "-d",
      &format!("@{}", tmp),
    ])
    .output()
    .unwrap();

  let raw = String::from_utf8(output.stdout).unwrap();
  let json: serde_json::Value = serde_json::from_str(&raw).unwrap_or_default();
  json["choices"][0]["message"]["content"]
    .as_str()
    .unwrap_or("")
    .to_string()
}
