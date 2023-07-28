/*
[Subtyping](https://doc.rust-lang.org/nomicon/subtyping.html)
is a relationship between types that allows statically typed languages to be a bit more flexible and permissive.
*/
trait Animal {
    fn snuggle(&self);
    fn eat(&mut self);
}

trait Cat: Animal {
    fn meow(&self);
}

trait Dog: Animal {
    fn bark(&self);
}

/*
fn love(pet: Animal) {
    pet.snuggle();
}

let mr_snuggles: Cat = ...;
love(mr_snuggles);         // ERROR: expected Animal, found Cat
*/

// With subtypes, we can tweak our overly strict static type system with a simple rule: anywhere a value of type T is expected, we will also accept values that are subtypes of T.
// The core problem is that this rule, naively applied, will lead to meowing Dogs. 
// That is, we can convince someone that a Dog is actually a Cat. 

struct TestDog;
impl Dog for TestDog {
    fn bark(&self) {
        println!("wolf");
    }
}

impl Animal for TestDog {
    fn eat(&mut self) {
        
    }

    fn snuggle(&self) {
        
    }
}

fn main() {
}
