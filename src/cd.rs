use std::path::{PathBuf,Path};
use std::env;
pub fn cd(args: Vec<String>, crr_dir: &PathBuf)
{
    //Here if just cd is given we have to go to the root dicertory, but here we would just reach our main location where our rust program exists.
    if args.len()==1
    {
        let dir = &crr_dir;
        assert!(env::set_current_dir(&dir).is_ok());
    }
    //Here cd command takes the input and if the path exists it changes its execution path. 
    else if args.len()==2
    {
        let dir = &args[1];
        if Path::new(dir).exists()
        {
            assert!(env::set_current_dir(&args[1]).is_ok());
        }
        else
        {
            println!("cd : Invalid Directory");
        }
    }
    else
    {
        println!("cd : Too many arguments");
    }
}