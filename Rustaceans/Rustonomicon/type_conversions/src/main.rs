#![feature(pointer_byte_offsets)]
mod dot_operator;

/*
Types can implicitly be coerced to change in certain contexts. 
These changes are generally just weakening of types, largely focused around pointers and lifetimes. 
They mostly exist to make Rust "just work" in more cases, and are largely harmless.
*/

trait Trait {}

fn foo<X: Trait>(x: X) {}

// impl<'a> Trait for &'a mut i32 {}
impl<'a> Trait for &'a i32 {}

fn main() {
    let x = &mut 0;
    // println!("{:p}: {}", x, *x);
    // foo(x);

    /* Casts */
    cast_notes()
}

// Lengths when casting raw slices
fn cast_notes() {
    let values: Vec<u16> = vec![1, 2, 3, 4];
    let slices = values.as_slice();
    let ptr1 = slices.as_ptr();

    let ptr2 = &slices as *const _ as *const u16;

    let arr: [u16;4] = [5, 6, 7, 8];    // 16 bits * 4 = 64
    let ptr3 = &arr as *const u16 as *const u8;

    unsafe {
        for x in 0..values.len() {
            println!("Heap value[ptr1]: {}", *ptr1.offset(x as isize));
        }

        println!("Heap value[ptr3]: {}", *ptr3.offset(0));
        println!("Heap value[ptr3]: {}", *ptr3.offset(1));
        println!("Heap value[ptr3]: {}", *ptr3.offset(2));
        println!("Heap value[ptr3]: {}", *ptr3.offset(3));
        println!("Heap value[ptr3]: {}", *ptr3.offset(4));
        println!("Heap value[ptr3]: {}", *ptr3.offset(5));
        println!("Heap value[ptr3]: {}", *ptr3.offset(6));
        println!("Heap value[ptr3]: {}", *ptr3.offset(7));

        println!("Stack: {}", *ptr2.byte_offset(8));
    }

}
