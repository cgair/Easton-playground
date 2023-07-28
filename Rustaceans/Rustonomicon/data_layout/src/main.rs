#![feature(pointer_byte_offsets)]
#![allow(unused, non_snake_case)]
mod repr_rust;
mod dynamically_zero_sized_type;

use std::mem::{size_of, size_of_val};

trait SomeTrait { }

// get pointer width (single unit of measurement of pointer width), 
// will be
// 4 bytes wide on 32-bit targets or
// 8 bytes wide on 64-bit targets
const WIDTH: usize = size_of::<&()>();
const DOUBLE_WIDTH: usize = 2 * WIDTH;

// This is also integrate with The Rust Reference chapter "Tyoe layout".
// See also <https://doc.rust-lang.org/reference/type-layout.html>
fn main() {
    /* Primitive Data Layout */
    println!("+---------------------------------------------------------------------------+");
    println!("|                        Primitive Data Layout                              |");
    println!("+---------------------------------------------------------------------------+");
    println!("            Primitive Data Layout(bool): {}", size_of::<bool>());
    println!("           Primitive Data Layout(u8/i8): {}", size_of::<u8>());
    println!("         Primitive Data Layout(u16/i16): {}", size_of::<u16>());
    println!("         Primitive Data Layout(u64/i64): {}", size_of::<u64>());
    println!("           Primitive Data Layout(usize): {}", size_of::<usize>());   // on a 32 bit target, this is 4 bytes and on a 64 bit target, this is 8 bytes.
    println!("       Primitive Data Layout(u128/i128): {}", size_of::<u128>());
    println!("             Primitive Data Layout(f32): {}", size_of::<f32>());
    println!("             Primitive Data Layout(f64): {}", size_of::<f64>());
    println!("            Primitive Data Layout(char): {}, note that a char is a 'Unicode scalar value'.", size_of::<char>());
    println!();
    /* Pointers and References Layout */
    // pointers to sized types are 1 width
    println!("+---------------------------------------------------------------------------+");
    println!("|                   Pointers and References Layout                          |");
    println!("+---------------------------------------------------------------------------+");
    println!("   Pointers and References Layout(&i32): {}", size_of::<&i32>());
    let boxed = Box::new(1);
    println!("    Pointers and References Layout(Box): {}", size_of_val(&boxed));   // Pointers to sized types have the same size and alignment as usize.
                    // pointers to DSTs are currently twice the size of the size of usize and have the same alignment.
    assert_eq!(WIDTH, size_of::<&i32>());
    assert_eq!(WIDTH, size_of::<&mut i32>());
    assert_eq!(WIDTH, size_of::<Box<i32>>());
    assert_eq!(WIDTH, size_of::<fn(i32) -> i32>());
    
    /* Array Layout */
    let arr = [0, 1, 2, 3];
    println!("                  Array Layout([i32;4]): {}", size_of_val(&arr));    // An array of [T; N] has a size of size_of::<T>() * N and the same alignment of T.

    /* Slice Layout */                                                              
    println!("                   Slice Layout(&[i32]): {}", size_of::<&[i32]>());   // The first 8 bytes is the actual pointer to the first element in the array (or part of an array the slice refers to)
                                                                                    // The second 8 bytes is the length of the slice.
    let vec = vec![0, 1, 2, 3];
    let slice = vec.as_slice();
    /*
                Stack                        Heap
            +----------------+           +----------+
    vec <-- | buffer pointer-|-----+---->|     0    |
            +----------------+     |     +----------+
            |   capacity(4)  |     |     |     1    |
            +----------------+     |     +----------+
            |    length(4)   |     |     |     2    |
            +----------------+     |     +----------+
                                   |     |     3    |
                                   |     +----------+
                                   |
            +----------------+     |
            | buffer pointer-|-----+
    slice   +----------------+
            |    length(4)   |
            +----------------+
    */                                  
    unsafe {
        /* [Reference cast to raw pointer](https://users.rust-lang.org/t/reference-cast-to-raw-pointer/52897) */
        println!("                     [Lab] Slice Layout: {}", size_of_val(&slice));
        let ptr = &slice as *const _ as *const *const i32;
        println!("+---------------------------------------------------------------------------+");
        println!("| The first 8 bytes is the actual pointer to the first element in the array |");
        println!("|     (or part of an array the slice refers to): {:p}             |", *ptr);
        println!("+---------------------------------------------------------------------------+");
        println!("| The second 8 bytes is the length of the slice: {:?}                          |", *ptr.offset(-1) as usize);     // OR ptr.offset(-1).read() as usize NOTE it's -1 cause Stack grows downwards(是否真的是因为这个?)
        println!("+---------------------------------------------------------------------------+");
    }
    println!("          Slice Layout(&[&dyn Trait;4]): {}", size_of::<[&dyn SomeTrait; 4]>());    

    /* str Layout */
    let str = "Hello, world!";
    println!("                        str Layout(str): {}", size_of_val(&str)); // String slices are a UTF-8 representation of characters that have the same layout as slices of type [u8].

    /* Tuple Layout */
    // Tuples do not have any guarantees about their layout.
    // NOTE unit tuple (()) is guaranteed as a zero-sized type to have a size of 0 and an alignment of 1.

    /* Trait Object Layout */
    println!("    Trait Object Layout(&dyn SomeTrait): {}", size_of::<&dyn SomeTrait>());   // The layout for a pointer to a trait object looks like this:
                                                                                            // The first 8 bytes points to the data for the trait object
                                                                                            // The second 8 bytes points to the vtable for the trait object
    println!("Trait Object Layout(Box<dyn SomeTrait>): {}", size_of::<Box<dyn SomeTrait>>());
    
    // pointers to unsized types are double-width because 
    // aside from pointing to data they need to do an extra bit of 
    // bookkeeping to also keep track of the data's length or point to a vtable
    // pointers to unsized types are 2 widths
    assert_eq!(DOUBLE_WIDTH, size_of::<&str>()); // slice
    assert_eq!(DOUBLE_WIDTH, size_of::<&[i32]>()); // slice
    assert_eq!(DOUBLE_WIDTH, size_of::<&dyn ToString>()); // trait object
    assert_eq!(DOUBLE_WIDTH, size_of::<Box<dyn ToString>>()); // trait object
    println!();

    println!("+---------------------------------------------------------------------------+");
    println!("|               What is the overhead of Rust's Option type?                 |");
    println!("+---------------------------------------------------------------------------+");
    // <https://stackoverflow.com/questions/16504643/what-is-the-overhead-of-rusts-option-type>
    
    show_size!(header);
    show_size!(i32);
    show_size!(&i32);
    show_size!(Box<i32>);
    show_size!(&[i32]);
    show_size!(Vec<i32>);
    show_size!(Result<(), Box<i32>>);
    println!("+---------------------------------------------------------------------------+");
    println!("|            Note that &i32, Box, &[i32], Vec<i32> all use the              |");
    println!("|           non-nullable pointer optimization inside an Option!             |");
    println!("+---------------------------------------------------------------------------+");
    println!("|     Furthermore, this optimization occurs in all \"Option-like\" enums      |"); 
    println!("+---------------------------------------------------------------------------+");
}

#[macro_export]
macro_rules! show_size {
    (header) => (
        println!("{:<22} {:>4}    {}", "Type", "T", "Option<T>");
    );
    ($t:ty) => (
        println!("{:<22} {:4} {:4}", stringify!($t), size_of::<$t>(), size_of::<Option<$t>>())
    )
}

