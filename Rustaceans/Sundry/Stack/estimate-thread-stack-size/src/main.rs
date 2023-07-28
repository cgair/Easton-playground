#![allow(non_upper_case_globals)]
// [Default Rust thread stack size is 2MB #17044](https://github.com/rust-lang/rust/issues/17044)
use std::thread;
use std::ptr::NonNull;

// you can guesstimate the stack size using code which allows you to find out the number of recursive calls that fit into your stack. 
// A neat hack on top of that: you can actually determine how much stack a single recursive call consumes: 
// just, for example, look at the address of the same variable in two consecutive calls.
static mut pa: NonNull<usize> = NonNull::<usize>::dangling();
static mut pb: NonNull<usize> = NonNull::<usize>::dangling();

#[allow(unconditional_recursion)]
fn foo(mut x: usize) {
    println!("iteration: {}", x);
    // The estimate of your stack size is the stack consumed by a single iteration times the number of the last iteration before the code segfaults.
    if x == 0 {
        unsafe {
            pa = NonNull::new(&mut x as *mut _).expect("ptr is null!");
            println!("x = 0: {:p}", pa);
        }
    } else if x == 1 {
        unsafe {
            pb = NonNull::new(&mut x as *mut _).expect("ptr is null!");
            println!("x = 1: {:p}", pb);
            // let space = pa.as_ptr() as usize - pb.as_ptr() as usize;
            let space = pa.as_ptr().offset_from(pb.as_ptr());
            println!("memory for one iteration = {} bytes", space * 8);
            // return;
        }
    }
    foo(x + 1);

}

fn main() {
    let t1 = thread::spawn(|| {
        foo(0);
    });
    let per = 448; 
    println!("per = {}, and estimate stack size = {} MB", per, 5036 * per / (1024 * 1024));

    t1.join().unwrap();
}
