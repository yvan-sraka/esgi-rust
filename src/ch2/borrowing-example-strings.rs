use std::io;
use std::fs::File;

fn display(input: &String) {
    println!("You typed: {}", input.trim());
}

fn main() -> io::Result<()> {
    let _msg = "Hello World";
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    display(&input);
    display(&input);
    Ok(())
}