use std::fs::File;
use std::env;

fn main()
{
	let args: Vec<String> = env::args().collect();
	touch(args);
}

fn touch(arg_s: Vec<String>)
{
    let name=&arg_s[1];
    let f=File::create(name);
    let _f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}