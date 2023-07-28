#![allow(unused)]
/**
 * On-Stack Dynamic Dispatch
 * 
 * To meets all the constraints Rust places on us:
 * 
 * All variables are initialized before using (in this case borrowing) them
 * Each variable only holds values of a single type. In our example, stdin is of type Stdin, file is of type File and readable is of type &mut dyn Read
 * Each borrowed value outlives all the references borrowed from it
 * 
 * 
 */
use std::{fs, io};

fn main() {
    let arg = "-";

    // These must live longer than `readable`, and thus are declared first:
    let (mut stdin_read, mut file_read);

    // We need to ascribe the type to get dynamic dispatch.
    let readable: &mut dyn io::Read = if arg == "-" {
        stdin_read = io::stdin();
        &mut stdin_read

        // &mut io::stdin()
        //      ^^^^^^^^^^^ creates a temporary which is freed while still in use
    } else {
        file_read = fs::File::open(arg).unwrap();
        &mut file_read
    };

    // Read from `readable` here.
}

// Disadvantages
// The code needs more moving parts than the Box-based version:
fn boxed_version(arg: &str) {
    // We still need to ascribe the type for dynamic dispatch.
    let readable: Box<dyn io::Read> = if arg == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(fs::File::open(arg).unwrap())
    };
    // Read from `readable` here.
}