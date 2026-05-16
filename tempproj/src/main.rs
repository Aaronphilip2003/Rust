use std::io;

fn main() {
    println!("Enter your input");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input = input.trim();

    println!("Hello {input}");
}