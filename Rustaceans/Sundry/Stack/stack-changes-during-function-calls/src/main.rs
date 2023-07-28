// two sum
pub fn sum(a: i32, b: i32) -> i32 {
    let c = a + b;
    c
}

// You need to analyze the stack changes during the function call step by step against the assembly code.
// for details see https://github.com/CGair23/weekend-md/blob/master/interview/Bytedance%20Rust.md
fn main() {
    let (x, y) = (5, 10);
    let z = sum(x, y);
    println!("x + y = {}", z);
}
