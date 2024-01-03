use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/*
open and read a file then handle the error if there's an error with match
*/
fn main() {
    let file = File::open("none_existing.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found {}", error)
            }
            _ => {
                panic!("Error opening file {}", error)
            }
        },
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line.unwrap())
    }
}
