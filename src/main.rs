use std::fs;
use std::path::Path;
use std::process::Command;

fn tree() -> Result<(), Box<dyn std::error::Error>> {
	for dir in fs::read_dir(Path::new("/"))? {
		let path = dir?.path();
		match fs::metadata(&path) {
			Ok(meta) => println!("{}: {:?}", path.display(), meta),
			Err(_) => println!("{}: Err", path.display())
		}
	}
	Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	tree()
}
