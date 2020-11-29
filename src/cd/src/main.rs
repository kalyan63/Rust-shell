use std::env;
use std::path::{PathBuf,Path};
fn main()
{
	let args: Vec<String> = env::args().collect();
	cd(args);
}
fn cd(args: Vec<String>, crr_dir: &PathBuf)
{
    if args.len()==1
    {
        let dir = &crr_dir;
        assert!(env::set_current_dir(&dir).is_ok());
    }
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