use std::{fs::File, io::Read};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <file> <keyword>", args[0]);
        return Ok(());
    }
    let fpath = &args[1];
    let keyword = &args[2];
    let content = read_content(fpath.as_str()).expect("Error reading file");
    let result = search(content.as_str(), keyword.as_str());
    if result.is_empty() {
        println!("No match found");
        return Ok(());
    }   
    for line in result {
        println!("{}", line);

    }
    Ok(())
}

fn read_content(fpath: &str) -> Result<String, std::io::Error> {
    let mut contentbuf = String::new();
    File::open(fpath)?.read_to_string(&mut contentbuf)?;
    Ok(contentbuf)
}

fn search<'a>(content: &'a str, keyword: &'a str) -> Vec<&'a str> { 
    content.lines().filter(|line| line.contains(keyword)).collect()
}
