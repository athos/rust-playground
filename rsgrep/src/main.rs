use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;

fn usage() {
    println!("rsgrep PATTERN FILENAME")
}

fn main() {
    let filename = match env::args().nth(2) {
        Some(filename) => filename,
        None => {
            usage();
            return;
        }
    };

    let file = match File::open(&filename) {
        Ok(file) => file,
        Err(e) => {
            println!("An error occurred while opening file {}:{}", filename, e);
            return;
        }
    };

    let input = BufReader::new(file);
    for line in input.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                println!("An error occurred while reading a line {}", e);
                return;
            }
        };
        println!("{}", line);
    }
}
