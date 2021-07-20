fn main() {
    let first_arg = std::env::args().nth(1).expect("no first arg");
    println!("Args: {}", first_arg);
}
