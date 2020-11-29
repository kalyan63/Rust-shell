use std::process::Command;
use std::process::Stdio;
pub fn execute(args: &Vec<String>)
{

    //Here you have give the path to the bin file in your computer 
    let path="/mnt/c/Users/skaly/Desktop/coding/rust/shell/src/bin/./".to_string();
    let mut commands=Vec::new();
    //Here we add the commands which we have implemented so that our binary files would run instead of the inbuild binaries. 
    commands.push(String::from("cat"));
    commands.push(String::from("grep"));
    commands.push(String::from("makdir"));
    commands.push(String::from("mv"));
    commands.push(String::from("cp"));
    commands.push(String::from("cmp"));
    commands.push(String::from("echo"));
    commands.push(String::from("rm"));
    commands.push(String::from("sort"));
    commands.push(String::from("wc"));
    let mut point=Vec::new();

    //Here we count the number of pipes so that we know how many times we should pipe
    for i in 0..args.len()
    {
        if args[i].eq("|")
        {
            point.push(i);
        }
    }
    point.push(args.len());
    let mut cmd=String::new();
    let mut argument=Vec::new();

    //Here we make sure that our binary files run if the given command is in the vector above.
    if commands.contains(&args[0])
    {
        cmd=path.to_owned().clone()+&args[0];
    }
    else
    {
        cmd=args[0].to_string();
    }

    //If there is no piping we run this.
    if point.len()==1
    {
        argument=args[1..].to_vec();
        let cmd1=Command::new(cmd).args(argument).stdout(Stdio::piped()).spawn().expect("Error");
        let cmd_out=cmd1.wait_with_output().expect("Failed to wait on cmd1");
        print!("{}",String::from_utf8_lossy(cmd_out.stdout.as_slice()));
    }
     
    //If there is piping we run this. The present implementation is just for single piping. we would improve it later.
    else
    {
        argument=args[1..point[0]].to_vec();
        let mut cmd1=Command::new(cmd).args(argument).stdout(Stdio::piped()).spawn().expect("Error");
        // Here we store the output generated by first command. 
        let cmd_out1=cmd1.stdout.expect("Failed to open ls stdout");
        if commands.contains(&args[point[0]+1])
        {
            cmd=path.to_owned().clone()+&args[point[0]+1];
        }
        else
        {
            cmd=args[point[0]+1].to_string();
        }
        //Here we run the second command with the output of first command as its input.
        if point[1]-point[0]>=2
        {
            argument=args[point[0]+2..point[1]].to_vec();
            cmd1=Command::new(cmd).args(argument).stdin(Stdio::from(cmd_out1)).stdout(Stdio::piped()).spawn().expect("Error");
            let output = cmd1.wait_with_output().expect("Failed to wait on cmd2");
	        print!("{}",String::from_utf8_lossy(output.stdout.as_slice()));
        }
        else
        {
            cmd1=Command::new(cmd).stdin(Stdio::from(cmd_out1)).stdout(Stdio::piped()).spawn().expect("Error");
            let output = cmd1.wait_with_output().expect("Failed to wait on cmd2");
	        print!("{}",String::from_utf8_lossy(output.stdout.as_slice()));
        }
    }
}