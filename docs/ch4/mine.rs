fn mine_mine_mine(s: String) {
    println!("{}", s);
}

fn main() {
    println!("Hello, world!");
    let mut str_on_heap = String::from("Hello, world!"); // Perfect for IO
    let mut str_on_stack = "Hello, world!";

    mine_mine_mine(str_on_heap); // <-- value moved here
    println!("{}", str_on_heap);

    // unsafe {
    //     // here there is no more memory checking rule
    // }
}