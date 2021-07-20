use std::io::{BufRead, BufReader, Error, ErrorKind};
use reqwest::StatusCode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let first_arg = std::env::args().nth(1).ok_or(Error::new(ErrorKind::NotFound, "File name is missing"))?;
    println!("Args: {}", first_arg);

    let urls = read_file(&first_arg)?;

    //urls.for_each(|url|
    //    let resp = client.get(&url).send()?;
    //);
    //urls.into_iter().for_each(fetch_status);
    let statuses: Result<Vec<_>, _> = urls.iter().map(fetch_status).collect();
    println!("{:?}", statuses?);

    Ok(())
}

fn read_file(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let file = std::fs::File::open(filename)?;
    let file = BufReader::new(file);

    //let urls: Vec<_> = file.lines().map(|line| line.unwrap()).collect();
    //let urls: Result<Vec<_>, _> = file.lines().collect()
    file.lines().collect()
}

fn fetch_status(url: &String) -> Result<StatusCode, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).send()?;
    Ok(resp.status())
}
