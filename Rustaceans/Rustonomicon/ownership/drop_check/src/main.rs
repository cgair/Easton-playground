#![feature(dropck_eyepatch)]
#![allow(unused)]
/**
 * ```rust
 * let x;
 * let y;
 * // desugaring to:
 * {
 *  let x;
 *  {
 *      let y;
 *  }
 * }
 * ```
 * There are some more complex situations which are not possible to desugar using scopes, 
 * but the order is still defined ‒ variables are dropped in the reverse order of their definition, 
 * fields of structs and tuples in order of their definition. 
 */

// type system isn't careful, it could accidentally make dangling pointers. 
// Consider the following simple program:


fn drop_check1() {
    struct Inspector<'a>(&'a u8);
    
    struct Word<'a> {
        inspector: Option<Inspector<'a>>,
        days: Box<u8>
    }

    let mut word = Word {
        inspector: None,
        days: Box::new(1)
    };
    // word.inspector = Some(Inspector(&word.days));   // 1. This program is totally sound and compiles.
                                                    // The fact that days does not strictly outlive inspector doesn't matter. 
                                                    // As long as the inspector is alive, so is days.
                                                    // 2. However if we add a destructor, the program will no longer compile!

    /*
    Implementing Drop lets the Inspector execute some arbitrary code during its death. 
    This means it can potentially observe that types that are supposed to live as long as it does actually were destroyed first.
    */
    impl<'a> Drop for Inspector<'a> {
        fn drop(&mut self) {
            println!("I was only {} days from retirement!", self.0);
        }
    }                                                
}

// However, both of the variants in `drop_check2` and `drop_check3` are rejected by the borrow checker during the analysis of fn main, saying that days does not live long enough.
// The reason is that the borrow checking analysis of main does not know about the internals of each Inspector's Drop implementation. 
// As far as the borrow checker knows while it is analyzing main, the body of an inspector's destructor might access that borrowed data.
// Therefore, the drop checker forces all borrowed data in a value to strictly outlive that value.
fn drop_check2() {
    struct Inspector<'a>(&'a u8, &'static str);
    
    impl<'a> Drop for Inspector<'a> {
        fn drop(&mut self) {
            println!("Inspector(_, {}) knows when *not* to inspect.", self.1);
        }
    }

    struct World<'a> {
        inspector: Option<Inspector<'a>>,
        days: Box<u8>,
    }

    let mut world = World {
        inspector: None,
        days: Box::new(1),
    };
    // world.inspector = Some(Inspector(&world.days, "gadget"));
    // Let's say `days` happens to get dropped first.
    // Even when Inspector is dropped, its destructor will not access the
    // borrowed `days`.
}

fn drop_check3() {
    struct Inspector<T>(T, &'static str);

    impl<T> Drop for Inspector<T> {
        fn drop(&mut self) {
            println!("Inspector(_, {}) knows when *not* to inspect.", self.1);
        }
    }

    // struct World<T> {
    //     inspector: Option<Inspector<T>>,
    //     days: Box<u8>,
    // }

    // let mut world = World {
    //     inspector: None,
    //     days: Box::new(1),
    // };
    // world.inspector = Some(Inspector(&world.days, "gadget"));
    // Let's say `days` happens to get dropped first.
    // Even when Inspector is dropped, its destructor will not access the
    // borrowed `days`.
}

/*
In the meantime, there is an unstable attribute (may_dangle) that one can use to assert (unsafely) that a generic type's destructor is guaranteed to not access any expired data, 
even if its type gives it the capability to do so.
To deploy it on the Inspector from above, we would write:
*/
fn drop_check4() {
    struct Inspector<'a>(&'a u8, &'static str);
    
    unsafe impl<#[may_dangle] 'a> Drop for Inspector<'a> {
        fn drop(&mut self) {
            println!("Inspector(_, {}) knows when *not* to inspect.", self.1);
        }
    }

    struct World<'a> {
        inspector: Option<Inspector<'a>>,
        days: Box<u8>,
    }

    let mut world = World {
        inspector: None,
        days: Box::new(1),
    };
    world.inspector = Some(Inspector(&world.days, "gadget"));
}

fn main() { }

