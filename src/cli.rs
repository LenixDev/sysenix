use std::sync::Mutex;

mod ai;
mod click;
mod parser;
mod prompt;
mod screenshot;
mod to_base64;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    println!("Usage: sy \"your request\"");
    return;
  }

  let user_request = args[1..].join(" ");
  println!("User: {}", user_request);

  loop {
    let bytes = screenshot::shot();
    let (w, h) = screenshot::dimensions(&bytes);
    let image = to_base64::to_base64(&bytes);
    let prompt = prompt::system(&user_request, w, h);
    let conversations: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());
    let response = ai::ask(&prompt, &conversations, &image);
    println!("Sysenix: {}", response);

    if let Some(action) = parser::parse(&response) {
      click::at(action.x as f64, action.y as f64);
      if action.is_last_step {
        break;
      }
      std::thread::sleep(std::time::Duration::from_millis(500));
    } else {
      break;
    }
  }
}
