# the motivation of this design
Consider this simple mistake that all of us who have used a non-GC'd language have made at one point:`
```rust
fn as_str(data: &u32) -> &str {
    // compute the string
    let s = format!("{}", data);

    // OH NO! We returned a reference to something that
    // exists only in this function!
    // Dangling pointer! Use after free! Alas!
    // (this does not compile in Rust)
    &s
}
```
Of course, Rust's story around ownership is much more complicated than just verifying that references don't escape the scope of their referent. 
That's because ensuring pointers are always valid is much more complicated than this. For instance in this code:
```rust
let mut data = vec![1, 2, 3];
// get an internal reference
let x = &data[0];

// OH NO! `push` causes the backing storage of `data` to be reallocated.
// Dangling pointer! Use after free! Alas!
// (this does not compile in Rust)
data.push(4);

println!("{}", x);
```