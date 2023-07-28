/**
 * Constructor
 * 
 * Rust's philosophy of being explicit: 
 * unlike C++, Rust does not come with a slew of built-in kinds of constructor.
 */
struct Foo {
    a: u8,
    b: u32,
    c: bool,
}

enum Bar {
    X(u32),
    Y(bool),
}

struct Unit;

#[test]
// There is exactly one way to create an instance of a user-defined type: 
// name it, and initialize all its fields at once
fn construcotrs() {
    let _foo = Foo { a: 0, b: 1, c: false };
    let _bar = Bar::X(0);
    let _empty = Unit;
}