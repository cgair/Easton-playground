#![allow(unused)]
// In my own words, polymorphism gives us the ability to present a single interface for potentially many different concrete types.
// There are several practical advantages to using polymorphism, but one of the biggest is code re-use.

// The examples to follow were made by disabling inlining and building in "debug" mode
trait Growler {
    fn growl(&self);
}

struct Lion;
impl Growler for Lion {
    #[inline(never)]
    fn growl(&self) {
        println!("Lion says GROWL!");
    }
}

struct Tiger;
impl Growler for Tiger {
    #[inline(never)]
    fn growl(&self) {
        println!("Tiger says GROWL!");
    }
}
struct Bear;
impl Growler for Bear {
    #[inline(never)]
    fn growl(&self) {
        println!("Bear says GROWL!");
    }
}

// by defining the Growler trait as a bound on the generic parameter T, 
// whatever type that’s passed in must implement that trait.
fn static_dispatch<T: Growler>(t: T) {
    t.growl();
}

fn dynamic_dispatch(t: &dyn Growler) {
    t.growl();
}

fn main() {
    static_dispatch(Lion);
    static_dispatch(Tiger);
    static_dispatch(Bear);

    // dynamic_dispatch(&Lion{});
    // dynamic_dispatch(&Tiger{});
    // dynamic_dispatch(&Bear{});
}
