use std::io::{BufRead, BufReader, Error, ErrorKind};
use reqwest::StatusCode;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct Stats {
    duration: Duration,
    number_of_bytes: usize,
}

impl Stats {
    fn aggregate(&mut self, other: &Stats) {
        self.duration += other.duration;
        self.number_of_bytes += other.number_of_bytes;
    }

    fn bytes_per_sec(&self) -> Option<f64> {
        match self.duration {
            0 => None,
            _ => (self.number_of_bytes as f64) / self.duration.as_secs_f64(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let first_arg = std::env::args().nth(1).ok_or(Error::new(ErrorKind::NotFound, "File name is missing"))?;
    println!("Args: {}", first_arg);

    let urls = read_file(&first_arg)?;

    //urls.for_each(|url|
    //    let resp = client.get(&url).send()?;
    //);
    //urls.into_iter().for_each(fetch_status);
    let stats: Result<Vec<_>, _> = urls.iter().map(fetch_status).collect();
    println!("{:?}", stats?);

    Ok(())
}

fn read_file(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let file = std::fs::File::open(filename)?;
    let file = BufReader::new(file);

    //let urls: Vec<_> = file.lines().map(|line| line.unwrap()).collect();
    //let urls: Result<Vec<_>, _> = file.lines().collect()
    file.lines().collect()
}

fn fetch_status(url: &String) -> Result<Stats, Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let client = reqwest::blocking::Client::new();
    //let resp = client.get(url).send()?;
    let resp = client.get(url).send()?;
    Ok(Stats {
        duration: start_time.elapsed(),
        //number_of_bytes: resp.content_length().ok_or(Error::new(ErrorKind::NotFound, "No content length"))? as usize,
        number_of_bytes: resp.text()?.len(),
    })
}
