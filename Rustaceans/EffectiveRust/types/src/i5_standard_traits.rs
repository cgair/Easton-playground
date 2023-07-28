/**
 * Rust encodes key behavioural aspects of its type system in the type system itself, through a collection of fine-grained standard traits that describe those behaviours.
 * Many of these traits will seem familiar to programmers coming from C++, corresponding to concepts such as copy-constructors, destructors, equality and assignment operators, etc.
 * <https://www.lurklurk.org/effective-rust/std-traits.html>
 */

// //////////////////////////////////// //
//             Auto Traits              //
// //////////////////////////////////// //
// <https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md#auto-traits>

// ////////////////// //
//    Send & Sync     //
// ////////////////// //

/*
Why most types are Sync without requiring any explicit synchronization? 
In the event we need to simultaneously mutate some data T across multiple threads, 
the compiler won't let us until we wrap the data in a Arc<Mutex<T>> or Arc<RwLock<T>>,
so the compiler enforces that explicit synchronization is used when it's needed.
*/
#[test]
fn send_sync() {
    use std::thread;
    // 1. We can pass many immutable references to the same data to many threads and we're guaranteed there are no data races 
    //    because as long as any immutable references exist Rust statically guarantees the underlying data cannot be mutated.
    let mut greeting = String::from("Hello");
    let greeting_ref = &greeting;
    thread::scope(|s| {
        // spawn 3 threads
        for n in 0..3 {
            s.spawn(move || {
                println!("{} {}", greeting_ref, n); // prints "Hello {n}"
            });
        }

        // line below could cause UB or data races but compiler rejects it
        // greeting += " world"; // ❌ cannot mutate greeting while immutable refs exist
    });

    // can mutate greeting after every thread has joined
    greeting += " world"; // ✅
    println!("{}", greeting); // prints "Hello world"

    // 2. We can pass a single mutable reference to some data to a single thread and we're guaranteed there will be no data races 
    //    because Rust statically guarantees aliased mutable references cannot exist and the underlying data cannot be mutated 
    //    through anything other than the single existing mutable reference.
    let mut greeting = String::from("Hello");
    let greeting_ref = &mut greeting;
    thread::scope(|s| {
        // greeting_ref moved into thread
        s.spawn(move || {
            *greeting_ref += " world";
            println!("{}", greeting_ref); // prints "Hello world"
        });

        // line below could cause UB or data races but compiler rejects it
        // greeting += "!!!"; // ❌ cannot mutate greeting while mutable refs exist
    }); 

    // can mutate greeting after the thread has joined
    greeting += "!!!"; // ✅
    println!("{}", greeting); // prints "Hello world!!!"   
}
