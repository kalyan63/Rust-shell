pub fn parse(line: String) -> Vec<String>
{
    let mut arg_s: Vec<String> = Vec::new();
    let mut f=0;
    let mut b=String::from("");
    //Here we parse the input if there is space between the words and all the words between " " is concidered as one word.
    for i in line.chars() {
        if i.eq(&' ') && f==0
        {
            b.push('`');
        }        
        else if i.eq(&'\"')
        {
            b.push('`');
            if f==1
            {
                f=0;
            }
            else{
                f=1;
            }
        }
        else if i.eq(&'\n')
        {
            b.push('`');
        }
        else
        {
            b.push(i)
        }
    }
    for i in b.split("`")
    {
        if !i.eq("") 
        {
        arg_s.push(i.trim().to_string());
        }
    }
    //Here we return the parsed string. 
    arg_s
}
