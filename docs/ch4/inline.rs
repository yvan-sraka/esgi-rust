// Type your code here, or load an example.
pub fn cube(num: i32) -> i32 {
    num * square(num)
}

#[inline(always)]
pub fn square(num: i32) -> i32 {
    num * num
}