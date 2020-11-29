use std::fs;
use std::path::Path;
pub fn run(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
	
	//Here we just print all the files and folders in a given path.
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