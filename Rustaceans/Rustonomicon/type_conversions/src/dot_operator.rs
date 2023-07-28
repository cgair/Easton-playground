/*
The dot operator will perform a lot of magic to convert types. 
It will perform 
    auto-referencing, 
    auto-dereferencing, 
    and coercion
until types match. 

[learn more](https://doc.rust-lang.org/nomicon/dot-operator.html)
*/

// fn do_stuff<T: Clone>(value: &T) {
//     let _cloned = value.clone();
// }

// remove `T: Clone` restriction
fn do_stuff<T>(value: &T) {
    let _cloned = value.clone();    // It would not be able to call by value, since there is no implementation of Clone for T. 
                                        // So the compiler tries to call by autoref. 
                                        // In this case, the function has the signature fn clone(&&T) -> &T since Self = &T. 
                                        // The compiler sees that &T: Clone, and then deduces that cloned: &T.
}

use std::sync::Arc;
struct Container<T>(Arc<T>);

// impl<T> Clone for Container<T> // #[derive(Clone)] does
// where 
//     T: Clone
// {
//     fn clone(&self) -> Self {
//         Self(Arc::clone(&self.0))
//     }
// }

impl<T> Clone for Container<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

fn clone_containers<T>(foo: &Container<i32>, bar: Container<T>) {
    let _foo_cloned = foo.clone();
    let _bar_cloned = bar.clone();
}
/*
What types are foo_cloned and bar_cloned? 
We know that Container<i32>: Clone, so the compiler calls clone by value to give foo_cloned: Container<i32>. 
However, bar_cloned actually has type &Container<T>. Surely this doesn't make sense - we added #[derive(Clone)] to Container, so it must implement Clone!
*/

use std::rc::Rc;
use std::boxed::Box;
fn method_lookup_algorithm() {
    let array: Rc<Box<[u8; 3]>> = Rc::new(Box::new([0u8;3]));
    let first_entry = array[0];
}
/*
How does the compiler actually compute array[0] when the array is behind so many indirections? 
    First, array[0] is really just syntax sugar for the Index trait - the compiler will convert array[0] into array.index(0). 
    Now, the compiler checks to see if array implements Index, so that it can call the function.
    Then, the compiler checks if Rc<Box<[T; 3]>> implements Index, but it does not, and neither do &Rc<Box<[T; 3]>> or &mut Rc<Box<[T; 3]>>. 
    Since none of these worked, the compiler **dereferences** the Rc<Box<[T; 3]>> into Box<[T; 3]> and tries again. 
    Box<[T; 3]>, &Box<[T; 3]>, and &mut Box<[T; 3]> do not implement Index, so it dereferences again. 
    [T; 3] and its **autorefs** also do not implement Index. It can't dereference [T; 3], so the compiler **unsizes** it, giving [T]. 
    Finally, [T] implements Index, so it can now call the actual index function.
*/

#[test]
fn maybe_compile() {
    do_stuff(&0);
}