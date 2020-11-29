use std::fs;
use std::env;
fn main()
{
	let args: Vec<String> = env::args().collect();
	mkdir(args);
}
fn mkdir(arg_s:Vec<String>)
{
    for i in 1..arg_s.len()
    {
        match fs::create_dir(&arg_s[i]) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(_) => {},
        }
    }
}