#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8
}

// fn complementary(color: &Color) -> Color {
//     Color {
//         r: 255 - color.r,
//         g: 255 - color.g,
//         b: 255 - color.b
//     }
// }

fn complementary_in_place(color: &mut Color) {
    color.r = 255 - color.r;
    color.g = 255 - color.g;
    color.b = 255 - color.b;
}

fn display(color: &Color) {
    println!("{:?}", color);
    // println!("{:x} {:x} {:x}", color.r, color.g, color.b);
}

fn main() {
    let mut red = Color { r: 255, g: 0, b: 0 };
    display(&red);
    complementary_in_place(&mut red);
    display(&red);
}
