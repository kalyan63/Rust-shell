use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::env;
pub fn cmp(args: &Vec<String>) {

	if args.len() <= 2 {
		println!("insufficient input arguments");
	}
	else if args.len()==3{
        if Path::new(&args[1]).exists(){
            if Path::new(&args[2]).exists(){

                let file1 = File::open(&args[1]).unwrap();
                // let file2 = File::open(&args[2]).unwrap();
                let contents1 = BufReader::new(file1);
                // let contents2 = BufReader::new(file2);
                let mut totallines1=0;
                let mut totallines2=0;
                for (index1, line1) in contents1.lines().enumerate() {
                    let lines1 = line1.unwrap(); 
                    let file2 = File::open(&args[2]).unwrap();
                    let contents2 = BufReader::new(file2);
                    for (index2, line2) in contents2.lines().enumerate() {
                            
                    let lines2 = line2.unwrap();
                    totallines1 = index1;//stores total number of lines in file2
                    totallines2 = index2;//stores total number of lines in file2 
                    if index1 == index2 && lines1 != lines2 {
                        println!("lines are different");
                    //different lines occurred	
                    break;
                    }
                }
                
                }
                if totallines1 != totallines2 {//wether the files have same number of lines
                        println!("different");
                }
            }
            else {
                 println!("path to file2 does not exist")
             }
            }
         else{
             println!("path to file1 does not exist");
         }

	    
}
else {
    println!("INVALID NUMBER OF INPUTS");
}
return;
}

fn main(){
	
	let args: Vec<String> = env::args().collect();
	cmp(&args);
}