#![allow(unused)]
/*
Data races are defined as:
    two or more threads concurrently accessing a location of memory
    one or more of them is a write
    one or more of them is unsynchronized

Safe Rust 保证不存在数据竞争 -> Data races are mostly prevented through Rust's ownership system: it's impossible to alias a mutable reference, so it's impossible to perform a data race. 
*/

// Only in conjunction with some other unsafe code can a race condition actually violate memory safety. 
use std::{
    thread,
    sync::Arc
};

fn main() {
    // TBD
}
