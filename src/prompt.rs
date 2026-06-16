pub fn system(user_request: &str) -> String {
  format!(
    "In this UI screenshot, what is the position of the element corresponding to the command \"{}\" (with point)?",
    user_request
  )
}