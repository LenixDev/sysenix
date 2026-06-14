use std::{process::Command, sync::Mutex};

pub fn ask(
  prompt: &str,
  conversation: &Mutex<Vec<(String, String)>>,
  image_base64: &str,
) -> String {
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
		r#"{{"role":"user","content":"{}","images":["{}"]}}"#,
		prompt.replace('"', "\\\"").replace('\n', "\\n"),
		image_base64
	));
  drop(history);

	let body = format!(
			r#"{{"model":"qwen2.5vl:7b","options":{{"num_ctx":16384}},"messages":[{}],"stream":false}}"#,
			messages_json
	);

	let tmp = "/tmp/sysenix_request.json";
	std::fs::write(tmp, &body).unwrap();

	let output = Command::new("curl")
			.args([
					"-s",
					"-X",
					"POST",
					"http://localhost:11434/api/chat",
					"-H",
					"Content-Type: application/json",
					"-d",
					&format!("@{}", tmp),
			])
			.output()
			.unwrap();

			let raw = String::from_utf8(output.stdout).unwrap();
			println!("raw response: {}", raw);
			let json: serde_json::Value = serde_json::from_str(&raw).unwrap_or_default();
			json["message"]["content"]
					.as_str()
					.unwrap_or("")
					.to_string()
}
