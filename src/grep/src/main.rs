use std::fs::File;
use std::path::Path;
use std::env;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::fs::OpenOptions;
fn main()
{
    let args: Vec<String> = env::args().collect();
    grep(&args);
}
pub fn grep(args: &Vec<String>) -> (){
if args.len()== 1 
{
// pattern to be detected is not entered
    println!("the pattern to be checked is not entered");
    return;
}
if args.len()==2 {
// we have to check from the user input
loop
{
    let mut checkin = String::new();
    std::io::stdin().read_line(&mut checkin).ok().expect("unable to enter line");
    let checkin = checkin.trim();
    if checkin.is_empty()
    {
        break;
    }
    if checkin.contains(&args[1])
    {
        println!("{}",checkin);
    }
}
   }  
    if args.len()>2 {
        if args.len()==4{
            if args[3] == ">"{
                println!("please enter the line to check");
                 let destiny = &args[args.len()-1];
                let mut checkin = String::new();
                let mut destination = std::fs::File::create(destiny).expect("create failed");
                std::io::stdin().read_line(&mut checkin).ok().expect("unable to enter line");
                // let mut to_check = &args[1];
             let checkin = checkin.trim();
             if checkin.contains(&args[1]){
                destination.write_all(checkin.as_bytes()).expect("unable to write");
             }
             else{
                 println!("input does not match with the pattern in command line");
                 return;
             }
        }
        else if args[3]==">>"{
            if Path::new(&args[args.len()-1]).exists(){
                println!("please enter the line to check");
                let mut checkin = String::new();
                let destiny = &args[args.len()-1];
                let mut destination =OpenOptions::new().append(true).open(destiny).expect( "cannot open file");
                std::io::stdin().read_line(&mut checkin).ok().expect("unable to enter line");
                // let mut to_check = &args[1];
             let checkin = checkin.trim();
             if checkin.contains(&args[1]){
                destination.write_all(checkin.as_bytes()).expect("unable to write");
             }
             else{
                 println!("input does not match with the pattern in command line");
                 return;
             }
            }
            else {
            println!("destination file to write output does not exist");
            }
        }
    }
        else if args.len()>4{
            if args[args.len()-2]==">"{
                let destiny = &args[args.len()-1];
                let mut destination = std::fs::File::create(destiny).expect("create failed");
                for i in 2..args.len()-2{
                    let file = &args[i];
                    // file to be checked for args[1]
                    if Path::new(&args[i]).exists(){   //checking wether the path for a given file exists or not 
                    // Open the file in read-only mode  
                    let checkfile = File::open(file).unwrap();
                    let content = BufReader::new(checkfile);
                    // Read the file line by line using the lines() iterator from std::io::BufRead.
                    for (_, line) in content.lines().enumerate() {
                    // checking line by line
                        let line = line.unwrap(); 
                        if line.contains(&args[1]){
                            destination.write_all(line.as_bytes()).expect("unable to write");  
                            destination.write_all("\n".as_bytes()).expect("unable to write");
                        }   
                    }
                    }
                    else{
                    // if the entered file name or path doesnot exist 
                        println!("file number {} does not exist",i)
                    }
                    }
            }
            else if args[args.len() - 2]==">>"{
                if Path::new(&args[args.len()-1]).exists(){
                let destiny = &args[args.len()-1];
                for i in 2..args.len()-2{
                    let file = &args[i];
                    let checkfile = File::open(file).unwrap();
                    // file to be checked for args[1]
                    if Path::new(&args[i]).exists(){   //checking wether the path for a given file exists or not 
                    // Open the file in read-only mode 
                    let content = BufReader::new(checkfile);
                    let mut destination =OpenOptions::new().append(true).open(destiny).expect( "cannot open file");
                    // Read the file line by line using the lines() iterator from std::io::BufRead.
                    for (_, line) in content.lines().enumerate() {
                    // checking line by line
                        let line = line.unwrap(); 
                        if line.contains(&args[1]){
                            destination.write_all(line.as_bytes()).expect("unable to write");  
                            destination.write_all("\n".as_bytes()).expect("unable to write");
                        }                        
                    }
                    }
                    else{
                    // if the entered file name or path doesnot exist 
                        println!("file number {} does not exist",i);
                    }
                    }
                }
                else{
                    println!("output file to write in to does not exist");
                } 
            }
        }
        else{
                for i in 2..args.len(){
                    let file = &args[i];
                    // file to be checked for args[1]
                    if Path::new(&args[i]).exists(){   //checking wether the path for a given file exists or not 
                    // Open the file in read-only mode 
                    let checkfile = File::open(file).unwrap();
                    let content = BufReader::new(checkfile);
                    // Read the file line by line using the lines() iterator from std::io::BufRead.
                    for (_, line) in content.lines().enumerate() {
                    // checking line by line
                        let line = line.unwrap(); 
                        if line.contains(&args[1]){
                        println!("{}",line);
                        }    
                    }
                    }
                    else
                    {
                    // if the entered file name or path doesnot exist 
                        println!("file number {} does not exist",i)
                    }
                   }
            }
    // single or multiple files
    }
return;
}