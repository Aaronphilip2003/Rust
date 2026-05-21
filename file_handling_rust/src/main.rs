use std::fs::OpenOptions;
use std::fs;
use std::io::Write;
fn main() {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("database.txt")
        .expect("Unable to open file");

    writeln!(file, "Hello Aaron").expect("Unable to write");

    let content = fs::read_to_string("database.txt").expect("Unable to read from file");

    println!("{}", content);
}
