// #![feature(pointer_byte_offsets)]

fn main() {
    let value = 0x12345678;
    println!("value: {:x}", value);
    println!("value: {:p}", &value);

    // [Why is casting a const reference directly to a mutable reference invalid in Rust?](https://stackoverflow.com/questions/53458784/why-is-casting-a-const-reference-directly-to-a-mutable-reference-invalid-in-rust)
    
    let ptr = &value as *const i32 as *mut u8;
    unsafe {
        println!("value: {:p}", ptr);
        println!("value: {:x}", *ptr);
        if *ptr == 0x78 {
            println!("Little endian: {:p}={:x}", ptr, *ptr);
            println!("               {:p}={:x}", ptr.offset(1), *ptr.offset(1));
            println!("               {:p}={:x}", ptr.offset(2), *ptr.offset(2));
            println!("               {:p}={:x}", ptr.offset(3), *ptr.offset(3));
        } else if *ptr == 0x12 {
            println!("Big endian:    {:p}={:x}", ptr, *ptr);
            println!("               {:p}={:x}", ptr.offset(1), *ptr.offset(1));
            println!("               {:p}={:x}", ptr.offset(2), *ptr.offset(2));
            println!("               {:p}={:x}", ptr.offset(3), *ptr.offset(3));
        }
    }
}