#![allow(unused, dead_code)]
use std::time::Instant;
/*
There are two kinds of reference:

    Shared reference: &
    Mutable reference: &mut
Which obey the following rules:

    A reference cannot outlive its referent
    A mutable reference cannot be aliased
*/
fn compute_v1(input: &u32, output: &mut u32) {
    if *input > 10 {
        *output = 1;
    }
    if *input > 5 {
        *output *= 2;    
    }
    // remember that `output` will be `2` if `input > 10`
}
// We would like to be able to optimize it to the following function:
/*
fn compute(input: &u32, output: &mut u32) {
    let cached_input = *input; // keep `*input` in a register
    if cached_input > 10 {
        // If the input is greater than 10, the previous code would set the output to 1 and then double it,
        // resulting in an output of 2 (because `>10` implies `>5`).
        // Here, we avoid the double assignment and just set it directly to 2.
        *output = 2;
    } else if cached_input > 5 {
        *output *= 2;
    }
}
*/
// Specifically, we need to worry about function arguments that make input and output overlap, such as compute(&x, &mut x).
/*
With that input, we could get this execution:

                    //  input ==  output == 0xabad1dea
                    // *input == *output == 20
if *input > 10 {    // true  (*input == 20)
    *output = 1;    // also overwrites *input, because they are the same
}
if *input > 5 {     // false (*input == 1)
    *output *= 2;
}
                    // *input == *output == 1

* In Rust we know this input should be impossible because &mut isn't allowed to be aliased.
[This is why alias analysis is important: it lets the compiler perform useful optimizations!](https://doc.rust-lang.org/nomicon/aliasing.html)
*/

// Move the only write to *output to the very end of our function. 
// This allows us to freely reorder the reads of *input that occur before it
fn compute_v2(input: &u32, output: &mut u32) {
    let mut temp = *output;
    if *input > 10 {
        temp = 1;
    }
    if *input > 5 {
        temp *= 2;
    }
    *output = temp;
}


fn main() {
    let a = 3u32;
    // compute(&a, &mut a);

    let in_v1 = 11u32;
    let mut out_v1 = 1u32;
    let start = Instant::now();
    compute_v1(&in_v1, &mut out_v1);
    println!("time elapsed: {:?}", start.elapsed());

    let in_v2 = 12u32;
    let mut out_v2 = 2u32;
    let start = Instant::now();
    compute_v1(&in_v2, &mut out_v2);
    println!("time elapsed: {:?}", start.elapsed());
}
