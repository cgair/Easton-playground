// Most of the time, we expect types to have a statically known and positive size. 
// This isn't always the case in Rust.

/**
 * Dynamically Sized Types (DSTs)
 * Rust supports Dynamically Sized Types (DSTs): types without a statically known size or alignment.
 * these types can only exist behind a pointer
 * 
 * There are two major DSTs exposed by the language:
 *     trait objects: dyn MyTrait
 *     slices: [T], str, and others
*/

// Can't be stored on the stack directly
struct MySuperSlice {
    info: u32,
    data: [u8] 
}   

struct MySuperSliceable<T: ?Sized> {
    info: u32,
    data: T
}

/*
Zero Sized Types (ZSTs)
Rust also allows types to be specified that occupy no space:
*/
struct Nothing;  // No fields = no size

// All fields have no size = no size
struct LotsOfNothing {
    foo: Nothing,
    qux: (),      // empty tuple has no size
    baz: [u8; 0], // empty array has no size
}

/*
Empty Types
Rust also enables types to be declared that cannot even be instantiated. 
*/
enum Void {} // No variants = EMPTY

#[test]
fn test_dst() {
    // such a type is largely useless without a way to construct it.
    // the following fails
    // let mss = MySuperSlice { info: 1, data: [1, 2, 3, 4]};
    
    let mss_sized: MySuperSliceable<[u8;8]> = MySuperSliceable {
        info: 1,
        data: [0;8]
    };

    let mss_dynamic: &MySuperSliceable<[u8]> = &mss_sized;
    println!("{}, {:?}", mss_sized.info, &mss_dynamic.data);
}

#[test]
fn test_zst() {
    let v: [u8;0] = [];
    use std::{mem, any::type_name};
    println!("{}", mem::size_of::<&[u8;0]>());
    println!("{}", mem::size_of_val(&v));
    println!("{}", type_name::<[u8;0]>());
    println!("{:?}", &v);
}

#[test]
fn test_empty() {
    let res: Result<u32, Void> = Ok(0);
 
    // let Ok(num) = res;
}