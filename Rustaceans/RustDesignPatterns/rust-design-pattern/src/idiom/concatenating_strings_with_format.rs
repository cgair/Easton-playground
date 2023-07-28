#![allow(unused)]
#![feature(test)]
/**
 * Build up strings using format! is succinct and readable. (convenient)
 * Especially where there is a mix of literal and non-literal strings.
 * 
 * Of course you can use the push and push_str methods on a mutable String, or using its + operator.
 */

fn say_hello(name: &str) -> String {
    // We could construct the result string manually.
    // let mut result = "Hello ".to_owned();
    // result.push_str(name);
    // result.push('!');
    // result

    // But using format! is better.
    format!("Hello, {}!", name)
}

// <https://stackoverflow.com/questions/71902146/are-string-literals-immutable>
fn literal_and_non_literal_strings() {
    let mut h = "Hello";    // However, the variable h is of type &str, so a pointer to a string slice, and it lives on the stack. 
    //                 h
    //     +-----------+
    //     |
    //     |
    // +---v-------+              
    // |           |              
    // |  "Hello"  |           
    // |           |     
    // +-----------+         
    h = "World";    // Therefore, it is totally valid for a to first point to the address of "Hello", and then to the address of "World".
    //     
    //               h +----------+
    //                            |
    //                            |
    // +---v-------+          +---v-------+             
    // |           |          |           |              
    // |  "Hello"  |          |  "World"  |         Neither "Hello" nor "World" ever change, but what h points to does.    
    // |           |          |           |
    // +-----------+          +-----------+

    println!("{}", h);

    // these literals are placed in an immutable section, the operating system prohibits modifying them.
    unsafe { (h.as_ptr() as *mut u8).write(42); }   // Segmentation fault
}

// Disadvantages
// It is usually not the most efficient way to combine strings - a series of push operations on a mutable string is usually the most efficient (especially if the string has been pre-allocated to the expected size).
// <https://stackoverflow.com/questions/63690623/why-is-the-format-macro-slower-than-pushing-into-a-string-directly>
fn use_push(username: &str, password: &str) -> String {
    let mut body = String::with_capacity(
        "grant_type=password&username=".len()
        + username.len()
        + "&password=".len()
        + password.len()
    );
    body.push_str("grant_type=password&username=");
    body.push_str(username);
    body.push_str("&password=");
    body.push_str(password);

    body
}

fn use_format(username: &str, password: &str) -> String {
    format!(
        "grant_type=password&username={}&password={}",
        username,
        password
    )
}

fn main() {
    // The executable binary produced by the rust compiler contains the string literals "Hello" and "World" in the read-only data section .rodata. 
    // ```cargo build --release && readelf -x .rodata target/release/concatenating_strings | grep Hello ```
    literal_and_non_literal_strings();  // See also <https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str>
}

#[cfg(test)]
mod tests {
    use super::*;
    
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_push(b: &mut Bencher) {
        b.iter(|| use_push("my_real_login", "with my real password"));
    }

    #[bench]
    fn bench_format(b: &mut Bencher) {
        b.iter(|| use_format("my_real_login", "with my real password"));
    }
}