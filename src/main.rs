mod instructions;
mod parser;
use std::io::BufRead;

fn iterator(filename: &str) -> impl Iterator {
    let fhandler = std::fs::File::open(filename).unwrap();
    let reader = std::io::BufReader::new(fhandler);
    reader.lines().map(|l| l.unwrap())
}

fn main() {
    let filepath = std::env::args().nth(1).expect("Give me an argument");
}
