use std::io::{BufRead, BufReader, Error, ErrorKind};
use reqwest::StatusCode;
use std::time::{Duration, Instant};
use reqwest::blocking::Client;

#[derive(Debug, PartialEq)]
struct Stats {
    duration: Duration,
    number_of_bytes: usize,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let first_arg = std::env::args().nth(1).ok_or(Error::new(ErrorKind::NotFound, "File name is missing"))?;
    println!("Args: {}", first_arg);

    let urls = read_file(&first_arg)?;

    //urls.into_iter().for_each(get);
    let initial_stat = Stats { duration: Duration::new(0, 0), number_of_bytes: 0 };
    let client = reqwest::blocking::Client::new();
    //let stats: Result<Vec<_>, _> = urls.iter().map(get).collect();
    //println!("{:?}", stats?);
    for url in urls {
        get(&client, &url, |req_stats| {
            initial_stat.aggregate(&req_stats);
            Ok(())
        })?;
    }
    println!("{:?}", initial_stat.bytes_per_sec());

    //stats?.iter().reduce(|memo, stat| memo.aggregate(&stat));
    //for stat in stats? {
    //    initial_stat.aggregate(&stat);
    //}

    Ok(())
}

fn read_file(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let file = std::fs::File::open(filename)?;
    let file = BufReader::new(file);

    //let urls: Vec<_> = file.lines().map(|line| line.unwrap()).collect();
    //let urls: Result<Vec<_>, _> = file.lines().collect()
    file.lines().collect()
}

//fn get(client: &Client, url: &String) -> Result<Stats, Box<dyn std::error::Error>> {
fn get<F>(client: &Client, url: &String, callback: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(Stats) -> Result<(), Box<dyn std::error::Error>>
{
    let start_time = Instant::now();
    //let resp = client.get(url).send()?;
    let resp = client.get(url).send()?;
    let stats = Stats {
        duration: start_time.elapsed(),
        number_of_bytes: resp.text()?.len(),
    };
    callback(stats)?;
    Ok(())
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
