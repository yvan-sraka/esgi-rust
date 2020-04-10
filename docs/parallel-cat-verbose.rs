// Parallel cat (verbose version)

use std::thread;

fn main() {
    // Create a vector to store all thread JoinHandle
    let mut children = vec![];
    // Read command-line arguments
    let args: Vec<String> = std::env::args().collect();
    // Iterate through args (but skip first: binary relative path)
    for arg in &args[1..] {
        // "magic": copy arg in memory before giving ownership to thread
        let filename = arg.clone();
        // Closure (an anonymous function or lambda function) ...
        // ... that "capture environment": take filename ownership
        let closure = move || {
            // Read file content ...
            let contents = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
            // ... and return it on the standard output
            return contents;
        };
        // Spawn a thread ...
        let child = thread::spawn(closure);
        // ... and store JoinHandle in a vector
        children.push(child);
    }
    // Wait for all thread to join (finish)
    for child in children {
        let res = child.join().unwrap();
        // Print file
        print!("{}", res);
    }
}
