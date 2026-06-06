use std::io::{self, Write};
use std::sync::Mutex;

use crate::locales::t;
use crate::parser;
use crate::prompt;
use crate::screenshot;
use crate::to_base64;
use crate::ai;
use crate::click;

pub fn start() {
  println!("{}", t("conversations_started"));

  let conversations: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());

  loop {
	  println!("{}", t("you"));
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let user_request = input.trim().to_string();

    if user_request.is_empty() {
      continue;
    }

    conversations.lock().unwrap().push(("user".to_string(), user_request.clone()));

	  println!("{}", t("thinking"));
    io::stdout().flush().unwrap();

    loop {
      let bytes = screenshot::shot();
      let (w, h) = screenshot::dimensions(&bytes);
      let image = to_base64::to_base64(&bytes);
      let prompt = prompt::system(&user_request, w, h);
      let response = ai::ask(&prompt, &conversations, &image);

      print!("\r{}: ", t("sysenix"));
      if let Some(action) = parser::parse(&response) {
        println!("clicking at ({}, {})", action.x, action.y);
        click::at(action.x as f64, action.y as f64);
        conversations.lock().unwrap().push(("assistant".to_string(), format!("Clicked at {},{}", action.x, action.y)));
        if action.is_last_step {
          break;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
      } else {
        println!("{}", response);
        conversations.lock().unwrap().push(("assistant".to_string(), response));
        break;
      }
    }
  }
}