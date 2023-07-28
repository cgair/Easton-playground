#![allow(unused)]
mod limits_of_lifetimes;
// The more complex cases where they don't coincide with scopes are [described below](https://doc.rust-lang.org/nomicon/lifetimes.html).
// One particularly interesting piece of sugar is that **each let statement implicitly introduces a scope.**
/* 
* Example: references that outlive referents
fn as_str(data: &u32) -> &str {
    let s = format!("{}", data);
    &s
}
desugars to

fn as_str<'a>(data: &'a u32) -> &'a str {
    'b: {
        let s = format!("{}", data);
        return &'a s;
    }
}
Since the contract of our function says the reference must outlive 'a, that's the lifetime we infer for the reference. 
Unfortunately, s was defined in the scope 'b, so the only way this is sound is if 'b contains 'a -- which is clearly false since 'a must contain the function call itself.
We have created a reference whose lifetime outlives its referent, which is literally the first thing we said that references can't do. 
*/

// We must produce an owned value inside the function to return it! 
fn as_str(data: &u32) -> String {
    format!("{}", data)
}

/*
* Example: aliasing a mutable reference

let mut data = vec![1, 2, 3];
let x = &data[0];
data.push(4);
println!("{}", x);

let mut data = vec![1, 2, 3];
let x = &data[0];
println!("{}", x);
// This is OK, x is no longer needed
data.push(4);

However, if the value has a destructor, the destructor is run at the end of the scope. 
And running the destructor is considered a use ‒ obviously the last one. So, this will not compile.

#[derive(Debug)]
struct X<'a>(&'a i32);

impl Drop for X<'_> {
    fn drop(&mut self) {}
}

let mut data = vec![1, 2, 3];
let x = X(&data[0]);
println!("{:?}", x);
data.push(4);
// Here, the destructor is run and therefore this'll fail to compile.
*/
#[derive(Debug)]
struct X<'a>(&'a i32);

impl Drop for X<'_> {
    fn drop(&mut self) {}
}

fn lifetime() {
    // let mut data = vec![1, 2, 3];
    // let x = X(&data[0]);
    // println!("{:?}", x);
    // data.push(4);
    // Here, the destructor is run and therefore this'll fail to compile.

    let mut data = vec![1, 2, 3];
    {
        let x = X(&data[0]);
        println!("{:?}", x);
    }
    data.push(4);
}