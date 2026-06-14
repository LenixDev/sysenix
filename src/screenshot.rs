use std::process::Command;

pub fn shot() -> Vec<u8> {
  let path = "/tmp/sysenix_screenshot.png";
  Command::new("screencapture")
    .arg("-x") // no sound
    .arg("-m") // main screen only
    .arg(path)
    .output()
    .unwrap();
  std::fs::read(path).unwrap()
}

pub fn dimensions(bytes: &[u8]) -> (u32, u32) {
  let w = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
  let h = u32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);
  (w, h)
}
