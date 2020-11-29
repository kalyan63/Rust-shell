use std::env;
fn main()
{
	let args: Vec<String> = env::args().collect();
	cd(args);
}
use std::fs;
use std::path::Path;
pub fn run(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
	if dir.is_dir() {
		for entries in fs::read_dir(dir)? {
				let entry = entries?;
				let file_name = entry
						.file_name()
						.into_string()
						.or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
				println!("{}", file_name);
		}
	}
	Ok(())
}