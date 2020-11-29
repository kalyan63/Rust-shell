use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::vec::Vec;
use std::{process,env};
use std::process::Command;
#[path="execute.rs"] mod execute;
#[path="parse.rs"] mod parse;
#[path="ls.rs"] mod ls;
#[path="cd.rs"] mod cd;
pub fn script(args: &Vec<String>) -> (){
let mut line_count = 0;
if args.len()== 1 {
//file name not entered
    println!("insufficient inputs");
    return;
}
if args.len()==2 
{
    //file to be sorted 
    if Path::new(&args[1]).exists()
    {   
        //checking wether the path for a given file exists or not 
        // Open the file in read-only mode 
        let checkfile = File::open(&args[1]).unwrap();
        let content = BufReader::new(checkfile);
            // Read the file line by line using the lines() iterator from std::io::BufRead.
            for (_, line) in content.lines().enumerate() {
                //adding every line to vector
                line_count = line_count + 1;
                let line = &line.unwrap();
                let main_dir=env::current_dir().unwrap();
                if line.trim().is_empty() 
                {
                    continue;
                }
                let arg_s=parse::parse((line).to_string());
                println!("{}",arg_s[0]);
                if arg_s[0].eq("exit")
                {
                    process::exit(0);
                }
                else if arg_s[0].eq("cd")
                {
                    cd::cd(arg_s,&main_dir);
                }
                else if arg_s[0].eq("pwd")
                {
                    println!("{:?}",env::current_dir().unwrap());
                }
                else
                {
                    execute::execute(&arg_s);
                }
                println!();
            }
    }
    else
    {
    // if the entered file name or path doesnot exist 
        println!("path to the file does not exist");
    }
}
else 
{
    println!("inputs exceeded");
}
}