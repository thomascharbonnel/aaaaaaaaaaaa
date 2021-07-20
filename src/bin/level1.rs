use std::io::{BufRead, BufReader};

fn main() {
    let first_arg = std::env::args().nth(1).expect("no first arg");
    println!("Args: {}", first_arg);

    read_file(&first_arg);
}

fn read_file(filename: &str) -> () {
    let file = std::fs::File::open(filename).unwrap();
    let file = BufReader::new(file);

    let urls: Vec<_> = file.lines().map(|line| line.unwrap()).collect();

    println!("Lines: {:?}", urls)
}
