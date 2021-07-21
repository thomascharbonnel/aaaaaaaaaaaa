use std::io::{BufRead, BufReader, Error, ErrorKind};
use reqwest::StatusCode;
use std::time::{Duration, Instant};
use std::sync::{Mutex, Arc};
use thiserror::Error;

#[derive(Debug, PartialEq)]
struct Stats {
    duration: Duration,
    number_of_bytes: usize,
}

#[derive(Debug, Error)]
enum CacheWarmerError
{
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
}

impl Stats {
    //fn aggregate(&mut self, other: &Stats) {
    fn aggregate(&self, other: &Stats) -> Stats {
        Stats {
            duration: self.duration + other.duration,
            number_of_bytes: self.number_of_bytes + other.number_of_bytes,
        }
    }

    fn bytes_per_sec(&self) -> Option<f64> {
        let duration = self.duration.as_secs_f64();
        match duration {
            d if d > 0.0 => Some((self.number_of_bytes as f64) / duration),
            _ => None,
        }
    }
}

fn main() -> Result<(), CacheWarmerError> {
    let first_arg = std::env::args().nth(1).ok_or(Error::new(ErrorKind::NotFound, "File name is missing"))?;
    println!("Args: {}", first_arg);

    let urls = read_file(&first_arg)?;

    let initial_stat = Stats { duration: Duration::new(0, 0), number_of_bytes: 0 };
    let shared_stat = Arc::new(Mutex::new(initial_stat));
    let mut threads = Vec::new();

    //urls.for_each(|url|
    //    let resp = client.get(&url).send()?;
    //);
    //urls.into_iter().for_each(fetch_status);
    for url in urls {
        let shared_stat = shared_stat.clone();
        let thread = std::thread::spawn(move || -> Result<(), CacheWarmerError> {
            let stat = fetch_status(&url)?;
            shared_stat.lock().unwrap().aggregate(&stat);
            Ok(())
        });
        threads.push(thread);
    }

    //threads.for_each(|thread| thread.join());
    for thread in threads {
        thread.join();
    }
    //let stats: Result<Vec<_>, _> = urls.iter().map(fetch_status).collect();
    //println!("{:?}", stats?);
    //stats?.iter().reduce(|memo, stat| memo.aggregate(&stat));
    //for stat in stats? {
    //    //initial_stat.aggregate(&stat);
    //}
    //println!("{:?}", initial_stat.bytes_per_sec());
    println!("{:?}", shared_stat.lock().unwrap().bytes_per_sec());

    Ok(())
}

fn read_file(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let file = std::fs::File::open(filename)?;
    let file = BufReader::new(file);

    //let urls: Vec<_> = file.lines().map(|line| line.unwrap()).collect();
    //let urls: Result<Vec<_>, _> = file.lines().collect()
    file.lines().collect()
}

//fn fetch_status(url: &String) -> Result<Stats, Box<dyn std::error::Error + Send>> {
fn fetch_status(url: &String) -> Result<Stats, CacheWarmerError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregate_with_zero() {
        let left = Stats { duration: Duration::new(0, 0), number_of_bytes: 0 };
        let right = Stats { duration: Duration::new(1, 0), number_of_bytes: 42 };

        assert_eq!(left.aggregate(&right), right);
    }

    #[test]
    fn aggregate() {
        let left = Stats { duration: Duration::new(42, 0), number_of_bytes: 42 };
        let right = Stats { duration: Duration::new(1, 0), number_of_bytes: 42 };
        let expected = Stats { duration: Duration::new(43, 0), number_of_bytes: 84 };

        assert_eq!(left.aggregate(&right), expected);
    }

    #[test]
    fn bytes_per_sec_empty() {
        let empty = Stats { duration: Duration::new(0, 0), number_of_bytes: 0 };
        assert_eq!(empty.bytes_per_sec(), None);
    }

    #[test]
    fn bytes_per_sec() {
        let empty = Stats { duration: Duration::new(10, 0), number_of_bytes: 10 };
        assert_eq!(empty.bytes_per_sec(), Some(1.0));
    }
}
