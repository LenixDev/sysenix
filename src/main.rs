use std::fs;
use std::path::Path;
use fs_extra;

fn tree() -> Result<(), Box<dyn std::error::Error>> {
	for dir in fs::read_dir(Path::new("/"))? {
		let path = dir?.path();
		match fs_extra::dir::get_size(&path) {
			Ok(size) => println!("{}: {}", path.display(), size),
			Err(_) => println!("{}: Error (permission denied?)", path.display()),
		}
	}
	Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  tree()
}
