#![allow(dead_code)]
/*
a situation where types or lifetimes are logically associated with a struct, but not actually part of a field. This most commonly occurs with lifetimes. 
For instance, the Iter for &'a [T] is (approximately) defined as follows:
```rust
struct Iter<'a, T: 'a> {
    ptr: *const T,
    end: *const T,
}
```

PhantomData is a special marker type. 
PhantomData consumes no space, but simulates a field of the given type for the purpose of static analysis.
*/

use std::marker;

struct Iter<'a, T: 'a> {
    ptr: *const T,
    end: *const T,
    _marker: marker::PhantomData<&'a T>,
}

fn main() {
    println!("Hello, world!");
}
