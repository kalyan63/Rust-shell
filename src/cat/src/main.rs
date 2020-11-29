use std::path::Path;
use std::env;
use std::io::Write;
use std::fs;
use std::vec::Vec;
use std::string::String;
use std::fs::OpenOptions;
fn main()
{
	let args: Vec<String> = env::args().collect();
	cat(args);
}
pub fn cat(args: Vec<String>)
{
    if args.len()==1
    {
        loop
        {
            let mut checkin = String::new();
            std::io::stdin().read_line(&mut checkin).ok().expect("unable to enter line");
            let checkin = checkin.trim();
            println!("{}",checkin);
            if checkin.is_empty()
            {
                break;
            }
            
        }
    }

    if args.len()>1{
        if Path::new(&args[args.len()-1]).exists(){
        if args[args.len() - 2]==">"{
            let destiny = &args[args.len() - 1];
            for i in 1..(args.len()-2){
                let file = &args[i];
                let contents = fs::read_to_string(file).expect("unable to read a file");
                let mut destination = std::fs::File::create(destiny).expect("create failed");
                destination.write_all(contents.as_bytes()).expect("unable to write");
            }
            return;
            
        }
        else if args[args.len()-2] == ">>"
        {
            let destiny = &args[args.len() - 1];
            for i in 1..(args.len()-2){
                let file = &args[i];
                let contents = fs::read_to_string(file).expect("unable to read");
                let mut destination =OpenOptions::new().append(true).open(destiny).expect( "cannot open file");
                destination.write_all(contents.as_bytes()).expect("unable to write");
            }
            return;
        }

    else {
        for i in 1..args.len()
        {
            let file = &args[i];
            let contents = fs::read_to_string(file).expect("unable to read");
            println!("{} : ",file);
            print!("{}",contents);
        }
        return;
    }
}
}
}