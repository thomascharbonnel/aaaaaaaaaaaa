use std::io::{BufRead, BufReader, Error, ErrorKind};

fn main() -> Result<(), std::io::Error> {
    let first_arg = std::env::args().nth(1).ok_or(Error::new(ErrorKind::NotFound, "File name is missing"))?;
    println!("Args: {}", first_arg);

    read_file(&first_arg)
}

fn read_file(filename: &str) -> Result<(), std::io::Error> {
    let file = std::fs::File::open(filename)?;
    let file = BufReader::new(file);

    //let urls: Vec<_> = file.lines().map(|line| line.unwrap()).collect();
    let urls: Result<Vec<_>, _> = file.lines().collect();
    let urls = urls?;

    println!("Lines: {:?}", urls);

    Ok(())
}
