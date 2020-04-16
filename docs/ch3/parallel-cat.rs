// Parallel cat

use std::thread;

fn main() {
    let mut children = vec![];
    let args: Vec<String> = std::env::args().collect();
    for arg in &args[1..] {
        let filename = arg.clone();
        children.push(thread::spawn(move || {
            std::fs::read_to_string(filename).expect("Something went wrong reading the file")
        }));
    }
    for child in children {
        print!("{}", child.join().unwrap());
    }
}
