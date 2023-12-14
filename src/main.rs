use std::env;
use std::fs;
use std::io::{self, BufRead};

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    day1::solve(read_lines(file_path.to_string()))
}

fn read_lines(file_path: String) -> day1::FileReader {
    let file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}


