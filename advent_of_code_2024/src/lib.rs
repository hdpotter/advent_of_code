use std::{fs::File, io::{self, BufRead}, path::Path};

// mostly copied from Rust By Example
pub fn read_lines<P: AsRef<Path>>(path: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}