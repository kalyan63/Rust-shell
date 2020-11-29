use std::env;
use std::vec::Vec;
use std::string::String;
fn main()
    {
        let args: Vec<String> = env::args().collect();
        rm(&args);
    }

fn rm(args: &Vec<String>) -> (){
    use std::io::ErrorKind;
	use std::path::Path;
	if args.len() == 1 {
		println!("insufficient input arguments");
	}
	if args.len() == 2 {
		if args[1] == "-r"{
			println!("file name not provided" );
		}
		else{
            let rmfile = &args[1];
            if Path::new(&args[1]).exists(){
				if Path::new(&args[1]).is_file(){
					match std::fs::remove_file(rmfile.clone()) {
                        Ok(file_in) => file_in,
                        Err(error) => match error.kind() {
                            ErrorKind::NotFound => {
                                println!(" Path does not exist");
                            
                            }
                            _other_error => {
                                println!("something unexpected happened");
                            }
                        },
                    };
				}
                else if Path::new(rmfile).is_dir() 
                {
					println!(" given input is a directory without  -r");
				}
		    }
        }
    }
	else if args.len() > 2{
		let mut rtag = false;//checks for -r tag
		let mut fileindex = 1; //index of begining file changes if -r tag is present 
		if args[1] == "-r" {
			rtag = true;
			fileindex = 2;
		}
		for i in fileindex..args.len(){
			let file = &args[i];
			println!("  {}", &args[i]);
			//checking wether the given path exists or not 
			if Path::new(file).exists(){
                //checking wether the given input is of a file 
				if Path::new(file).is_file(){
                    match std::fs::remove_file(file.clone()) {
                        Ok(file_in) => file_in,
                        Err(error) => match error.kind() {
                            ErrorKind::NotFound => {
                                println!(" Path does not exist");
                            }
                            _other_error => {
                                println!("something unexpected happened");
                            }
                        },
		            };
	            }
				else if Path::new(file).is_dir(){
                    //checking wether the input is a directory 
                    if rtag==false {
					    println!("path is a directory and -r is not found in the input");
                    }
                    else if rtag == true 
                    {
                        // use std::io::ErrorKind;
                        match std::fs::remove_dir_all(file.clone()) {
                            Ok(file_in) => file_in,
                            Err(error) => match error.kind() {
                                ErrorKind::NotFound => {
                                    println!("path does not exist");
                                    return
                                }
                                _other_error => {
                                    println!("something unexpected happened");
                                    return
                                }
                            },
                        };
				    }
			    }
		    }
            else
            {
				println!("path to given input file does not exist");
			}
		}
		
	}
	return;
}
