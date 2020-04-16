// Iterative cat

use std::thread;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    for filename in &args[1..] {
        let contents = std::fs::read_to_string(filename).expect("Something went wrong reading the file: {}");
        print!("{}", contents);
    }
}
