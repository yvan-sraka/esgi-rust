fn max_top3(top: &[u8; 3]) -> u8 {
    if top[0] > top[1] {
        if top[0] > top[2] top[0] else top[2]
    } else {
        if top[1] > top[2] top[1] else top[2]
    }
}

// https://doc.rust-lang.org/std/primitive.array.html

fn main() {
    let mut array: [u8; 3] = [23, 10, 50];
    array[2] = 12;
    // println!("{}", max_top3(&array));
    for elem in array.iter() {
        println!("{}", elem);
    }
}
