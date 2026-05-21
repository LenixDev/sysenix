use tray_icon::{TrayIconBuilder, menu::{Menu, MenuItem, MenuEvent}};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::WindowId;

struct App {
  _tray: tray_icon::TrayIcon,
}

impl ApplicationHandler for App {
  fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

  fn window_event(&mut self, _event_loop: &ActiveEventLoop, _id: WindowId, _event: WindowEvent) {}

  fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
    if let Ok(event) = MenuEvent::receiver().try_recv() {
      println!("menu clicked: {:?}", event);
      event_loop.exit();
    }
  }
}

fn main() {
  let event_loop = EventLoop::new().unwrap();

  let menu = Menu::new();
  let quit = MenuItem::new("Quit", true, None);
  menu.append(&quit).unwrap();

  let tray = TrayIconBuilder::new()
    .with_title("🤖")
    .with_menu(Box::new(menu))
    .build()
    .unwrap();

  let mut app = App { _tray: tray };
  event_loop.run_app(&mut app).unwrap();
}