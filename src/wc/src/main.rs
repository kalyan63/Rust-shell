use std::fs::File;
use std::path::Path;
use std::env;
use std::io::{BufRead, BufReader};
use std::vec::Vec;
use std::io::Write;
use std::fs::OpenOptions;
fn main(){	
	let args: Vec<String> = env::args().collect();
	sort(&args);
}
fn sort(args: &Vec<String>) -> (){
let mut line_count = 0;
let mut words:Vec<String> = Vec::new();
if args.len()== 1 {
//file name not entered
let mut inwords:Vec<String> = Vec::new();
let mut countline = 0;
    loop
    {   
        let mut checkin = String::new();
        std::io::stdin().read_line(&mut checkin).ok().expect("unable to enter line");
        countline = countline+1;
        let checkin = checkin.trim();
        if checkin.is_empty()
        {
            break;
        }
        inwords.push(checkin.to_string());
        let strings:Vec<&str> = checkin.split(' ').collect();
        for s in strings {
            inwords.push(s.to_string());   
        } 
        
    }
    let words = inwords.len()-1;
    countline-=1;
    println!("line_count is : {}",countline);
    println!("words_count is : {}",words); 
    return;
}
if args.len()==2 
{
    //file to be sorted 
    if Path::new(&args[1]).exists()
    {   //checking wether the path for a given file exists or not 
    // Open the file in read-only mode 
    let checkfile = File::open(&args[1]).unwrap();
    let content = BufReader::new(checkfile);
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in content.lines().enumerate() 
        {
        //adding every line to vector
            line_count = line_count + 1;
            let line = line.unwrap();
            let strings:Vec<&str> = line.split(' ').collect();
            for s in strings 
            {
                words.push(s.to_string());   
            } 
        }
    }
    else{
    // if the entered file name or path doesnot exist 
        println!("path to the file does not exist");
        return;
    }
}
else if args.len()== 4
{
if args[2]==">"
    {
        let destiny = &args[args.len()-1];
        let mut destination = std::fs::File::create(destiny).expect("create failed");
        if Path::new(&args[1]).exists()
        {   //checking wether the path for a given file exists or not 
            // Open the file in read-only mode 
            let checkfile = File::open(&args[1]).unwrap();
            let content = BufReader::new(checkfile);
            // Read the file line by line using the lines() iterator from std::io::BufRead.
            for (_, line) in content.lines().enumerate()
            {
            //adding every line to vector
                line_count = line_count + 1;
                let line = line.unwrap();
                let strings:Vec<&str> = line.split(' ').collect();
                for s in strings 
                {
                    words.push(s.to_string());   
                } 
            }
            let wordcount = words.len();
            destination.write_all(line_count.to_string().as_bytes()).expect("unable to write");
            destination.write_all("\n".as_bytes()).expect("unable to write");
            destination.write_all(wordcount.to_string().as_bytes()).expect("unable to write line count");
            return;
        }
        else
        {
            // if the entered file name or path doesnot exist 
            println!("path to the file does not exist");
            return;
        }
    }
    if args[2]==">>"
    {
        if Path::new(&args[3]).exists()
        {
        let destiny = &args[args.len()-1];
        let mut destination = OpenOptions::new().append(true).open(destiny).expect( "cannot open file");
        if Path::new(&args[1]).exists()
            {   //checking wether the path for a given file exists or not 
                // Open the file in read-only mode 
                let checkfile = File::open(&args[1]).unwrap();
                let content = BufReader::new(checkfile);
                // Read the file line by line using the lines() iterator from std::io::BufRead.
                for (_, line) in content.lines().enumerate() 
                {
                //adding every line to vector
                    line_count = line_count + 1;
                    let line = line.unwrap();
                    let strings:Vec<&str> = line.split(' ').collect();
                    for s in strings 
                    {
                        words.push(s.to_string());   
                    } 
                }
                let wordcount = words.len();
                destination.write_all(line_count.to_string().as_bytes()).expect("unable to write");
                destination.write_all("\n".as_bytes()).expect("unable to write");
                destination.write_all(wordcount.to_string().as_bytes()).expect("unable to write line count");
                return;
            }
            else
            {
                // if the entered file name or path doesnot exist 
                println!("path to the file does not exist");
                return;
            }
        }
        else
        {
            println!("output file to write in to does not exist");
            return;
        }
    }
}
else 
{
    println!("inputs exceeded");
} 
println!("line_count is : {}",line_count);
println!("words_count is : {}",words.len());
return;
}