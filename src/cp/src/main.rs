use std::path::Path;
use std::io::Write;
use std::fs;
use std::env;

fn main()
{
	let args: Vec<String> = env::args().collect();
	cp(args);
}

pub fn cp(args: Vec<String>)
{
    if args.len()<=2 
    {
        println!("cp : Not enough arguments");
    }
    else if args.len()==3
    {
        let fr = &args[1];
        let fw = &args[2];
        if Path::new(fr).exists()
        {
            let content = fs::read_to_string(fr).expect("unable to read ");
            let mut destination = std::fs::File::create(fw).expect("create failed");
            destination.write_all(content.as_bytes()).expect("unable to write");
        }  
        else
        {
            println!("cp: {} file doesn't exists",fr);
        }
    }
    else
    {
        println!("To many arguments");
    }
}