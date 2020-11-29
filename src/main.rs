use std::io::{stdin,stdout,Write};
use std::{process,env};
use std::path::Path;

//Here we use the functions built in other rust files.
mod execute;
mod parse;
mod script;
mod ls;
mod cd;

fn main() {
    let main_dir=env::current_dir().unwrap();
    loop
    {
        let current_dir = env::current_dir().unwrap();
        println!("coolie~code=>{:?}",current_dir);
        print!("=> ");
        stdout().flush().unwrap();
        let mut input= String::new();
        stdin().read_line(&mut input).unwrap();
        if input.trim().is_empty() 
        {
            continue;
        }
        //Here we parse the given input into Vec<String> form 
        let arg_s=parse::parse(input);
        if arg_s[0].eq("exit")
        {
            process::exit(0);
        }
        else if arg_s[0].eq("cd")
        {
            cd::cd(arg_s,&main_dir);
        }

        //Here ls has been commented since our implementation doesn't support piping, since we have not created the binary for ls.
        // You can uncommnet this if you don't want to pipe ls.
        // else if arg_s[0].eq("ls")
        // {
        //     if let Err(e) = &ls::run(Path::new(".")) 
        //     {
        //         println!("{}", e);
        //     }    
        // }
        else if arg_s[0].eq("pwd")
        {
            println!("{:?}",env::current_dir().unwrap());
        }

        // This is used to run the bash command.
        else if arg_s[0].eq("bash")
        {
            script::script(&arg_s)
        }

        //Here we use the binaries to run the commands and also we can run other binaries. 
        else
        {
            execute::execute(&arg_s);
        }
        println!();
    }
}

