use std::fs::File;
use std::path::Path;
use std::env;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::vec::Vec;
use std::string::String;
use std::fs::OpenOptions;
fn main(){
	
	let args: Vec<String> = env::args().collect();
	sort(&args);
}

pub fn sort(args: &Vec<String>) -> (){
let mut v:Vec<String> = Vec::new();
if args.len()== 1 {
    let mut s:Vec<String> = Vec::new();
    loop
    {
        let mut checkin = String::new();
        std::io::stdin().read_line(&mut checkin).ok().expect("unable to enter line");
        let checkin = checkin.trim();
        s.push(checkin.to_string());
        if checkin.is_empty()
        {
            break;
        }
    }
    s.sort();
    //file name not entered
    s.sort();//sorts all the lines in the vector 
    // println!("{:?}",S);
    for i in 0..s.len()
    {
        println!("{}",s[i]);
    }
    return;
}

if args.len()==2 {
    if args[1]==">"{
        println!("invalid input format");
    }
    else{
        // let file = &args[1];
        //file to be sorted 
        if Path::new(&args[1]).exists(){   
            //checking wether the path for a given file exists or not 
            // Open the file in read-only mode 
            let checkfile = File::open(&args[1]).unwrap();
            let content = BufReader::new(checkfile);
            // Read the file line by line using the lines() iterator from std::io::BufRead.
            for (_, line) in content.lines().enumerate() {
            //adding every line to vector
                let line = line.unwrap(); 
                v.push(line.to_string());    
            }
            v.sort();//sorts all the lines in the vector 
            for i in 0..v.len()
            {
                println!("{}",v[i]);
            }
            return;
        }
        else{
            // if the entered file name or path doesnot exist 
            println!("path to the file does not exist");
        }
    }
}
else if args.len()==4{
    if args[2] == ">"{
        // let file = &args[3];
        let mut destination = std::fs::File::create(&args[3]).expect("create failed");
        //file to be sorted 
        if Path::new(&args[3]).exists(){   
            //checking wether the path for a given file exists or not 
            // Open the file in read-only mode 
            let checkfile = File::open(&args[1]).unwrap();
            let content = BufReader::new(checkfile);
        
            // Read the file line by line using the lines() iterator from std::io::BufRead.
            for (_, line) in content.lines().enumerate() {
            //adding every line to vector
                let line = line.unwrap(); 
                v.push(line.to_string());    
            }
            v.sort();

            for i in 0..v.len(){
                // let mut destination = std::fs::File::create(fw).expect("create failed");
                destination.write_all(v[i].to_string().as_bytes()).expect("unable to write");
            }
        }
    }
    else if args[2]==">>" {
        // let file = &args[3];
        let mut destination = OpenOptions::new().append(true).open(&args[3]).expect( "cannot open destination file");
        //file to be sorted 
        if Path::new(&args[1]).exists(){   
            //checking wether the path for a given file exists or not 
            // Open the file in read-only mode 
            let checkfile = File::open(&args[1]).unwrap();
            let content = BufReader::new(checkfile);
        
            // Read the file line by line using the lines() iterator from std::io::BufRead.
            for (_, line) in content.lines().enumerate() {
            //adding every line to vector
                let line = line.unwrap(); 
                v.push(line.to_string());    
            }
            v.sort();
            for i in 0..v.len()
            {
                // let mut destination = std::fs::File::create(fw).expect("create failed");
                destination.write_all(v[i].to_string().as_bytes()).expect("unable to write");
            }
        }
        else {
            println!("given input file to be sorted does not exist");
        }
    }
}
return;
}  