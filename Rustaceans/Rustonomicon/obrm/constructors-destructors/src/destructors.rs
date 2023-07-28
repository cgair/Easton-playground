/**
 * [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html)
 * ```rust
 * pub trait Drop {
 *     fn drop(&mut self);
 * }
 * ```
 * 
 * If a destructor must be run manually, 
 * such as when implementing your own smart pointer, 
 * [std::ptr::drop_in_place](https://doc.rust-lang.org/std/ptr/fn.drop_in_place.html) can be used.
 */ 

use std::alloc::{Allocator, Global, GlobalAlloc, Layout};
use std::ptr::{Unique, NonNull, drop_in_place};
use std::mem;

pub struct MyBox<T>(Unique<T>);

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        unsafe {
            drop_in_place(self.0.as_ptr());
            let c: NonNull<T> = self.0.into();
            Global.deallocate(c.cast(), Layout::new::<T>());
        }
    }
}

struct SuperBox<T> { my_box: MyBox<T> }

// After drop is run, Rust will recursively try to drop all of the fields of self.
// After we deallocate the box's ptr in SuperBox's destructor, 
// Rust will happily proceed to tell the box to Drop itself and everything will blow up with use-after-frees and double-frees.
impl<T> Drop for SuperBox<T> {
    fn drop(&mut self) {
        unsafe {
            // Hyper-optimized: deallocate the box's contents for it
            // without `drop`ing the contents
            let c: NonNull<T> = self.my_box.0.into();
            Global.deallocate(c.cast::<u8>(), Layout::new::<T>());
        }
    }
}

// Note that the recursive drop behavior applies to all structs and enums regardless of whether they implement Drop. 

// The **classic safe solution to overriding recursive drop** 
// and allowing moving out of Self during drop is to use an Option:
struct OverideSuperBox<T> { my_box: Option<MyBox<T>> }

impl<T> Drop for OverideSuperBox<T> {
    fn drop(&mut self) {
        unsafe {
            // Hyper-optimized: deallocate the box's contents for it
            // without `drop`ing the contents. Need to set the `box`
            // field as `None` to prevent Rust from trying to Drop it.
            let my_box = self.my_box.take().unwrap();
            let c: NonNull<T> = my_box.0.into();
            Global.deallocate(c.cast(), Layout::new::<T>());
            mem::forget(my_box);
        }
    }
}