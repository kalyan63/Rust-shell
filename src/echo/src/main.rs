use std::env;
fn main()
{
	let args: Vec<String> = env::args().collect();
	echo(args);
}

pub fn echo(arg_s:Vec<String>)
{
    for i in 1..arg_s.len()
    {
        print!("{} ",arg_s[i]);
    }
    println!();
}