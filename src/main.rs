use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    let _ = 

    print!("Enter something: ");

    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("you entered: {}", input.trim());
}
