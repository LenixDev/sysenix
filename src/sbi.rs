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
      println!("item clicked: {:?}", event);
      event_loop.exit();
    }
  }
}

fn load_icon(path: &str) -> tray_icon::Icon {
  let file = std::fs::File::open(path).unwrap();
  let decoder = png::Decoder::new(file);
  let mut reader = decoder.read_info().unwrap();
  let mut buf = vec![0; reader.output_buffer_size()];
  let info = reader.next_frame(&mut buf).unwrap();
  let bytes = &buf[..info.buffer_size()];
  tray_icon::Icon::from_rgba(bytes.to_vec(), info.width, info.height).unwrap()
}

pub fn main() {
	let event_loop = EventLoop::new().unwrap();
  let menu = Menu::new();

	// Add the quit SBI
  let quit = MenuItem::new("Quit", true, None);
  menu.append(&quit).unwrap();

	// Load the SBI app icon
	let icon = load_icon("assets/favicon-light.png");

	// Init the SBI app
  let tray = TrayIconBuilder::new()
		.with_icon(icon)
    .with_menu(Box::new(menu))
    .build()
    .unwrap();

	// Run the SBI
  let mut app = App { _tray: tray };
  event_loop.run_app(&mut app).unwrap();
}