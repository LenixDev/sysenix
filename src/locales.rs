pub fn t(key: &str) -> &str {
  match key {
    "sysenix" => "Sysenix",
    "ask_sysenix" => "Ask Sysenix...",
    "quit" => "Quit",
    _ => key,
  }
}
