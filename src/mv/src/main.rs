use std::env;
// use std::io::Read;
use std::io::Write;
use std::io::ErrorKind;
use std::fs;
fn main(){
	
	let args: Vec<String> = env::args().collect();
	mv(&args);
}

pub fn mv(args: &Vec<String>) -> (){


    if args.len() == 1{
       println!("insufficient input files");
       return;
    }
    if args.len() ==2 {
       println!("destination file not found");
       return;
    }
    // let file = &args[1];
    // let destiny = &args[2];
    let _file = match std::fs::File::open(&args[1]) {
		Ok(file_in) => file_in,
		Err(error) => match error.kind() {
			ErrorKind::NotFound => {
				println!("source file not found");
				return;
			}
			_other_error => {
				println!("unexpected error");   
				return;
			}
		},
	};
    let _file2 = match std::fs::File::create(&args[2]) {
        Ok(file_in) => file_in,
		// Err(error) => {return}
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("destination file not found");
                return;
            }
            _other_error => {
                println!("unexpected error");
                return;
            }
        },
    };
    let content = fs::read_to_string(&args[1]).expect("unable to read ");
    let mut destination = std::fs::File::create(&args[2]).expect("create failed");
    destination.write_all(content.as_bytes()).expect("unable to write");
    std::fs::remove_file(&args[1]).expect("file does not exist ");
    return;
}