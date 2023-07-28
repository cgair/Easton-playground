/**
 * A library author want to add public fields to a public struct or new variants to an enum without breaking backwards compatibility.
 * 
 * Rust offers two solutions to this problem:
 * 
 * [1] Use [#[non_exhaustive]](https://github.com/rust-lang/rfcs/blob/master/text/2008-non-exhaustive.md) on structs, enums, and enum variants.
 * [2] You may add a private field to a struct to prevent it from being directly instantiated or matched against
 * 
 * On structs, #[non_exhaustive] allows adding additional fields in a backwards compatible way. It will also prevent clients from using the struct constructor, even if all the fields are public. 
 * 
 */

mod non_exhaustive_gate {
    // The most common use for non-exhaustive structs is config types.
    #[derive(Debug)]
    #[non_exhaustive]
    pub struct Config {
        pub window_width: u16,
        pub window_height: u16,
        // assume we make the following addition:
        // pub is_fullscreen: bool,    // Now, code that constructs the struct, like below, will fail to compile:
                                    // let config = Config { window_width: 640, window_height: 480 };
    }

    fn inside_config() {
        // We can still construct our config within the defining crate like so:
        let config = Config { window_width: 640, window_height: 480 };
        println!("{:?}", config);
    }

    // Public struct.
    #[non_exhaustive]
    pub struct S {
        pub foo: i32,
    }

    #[non_exhaustive]
    pub enum AdmitMoreVariants {
        VariantA,
        VariantB,
        #[non_exhaustive]
        VariantC { a: String }
    }
}

use non_exhaustive_gate::{S, AdmitMoreVariants};
fn print_matched_variants(s: S) {
    // Because S is `#[non_exhaustive]`, it cannot be named here and
    // we must use `..` in the pattern.
    let S { foo: _, .. } = s;  //  without the .., this code will fail to compile.


    let some_enum = AdmitMoreVariants::VariantA;
    match some_enum {
        AdmitMoreVariants::VariantA => println!("it's an A"),
        AdmitMoreVariants::VariantB => println!("it's a b"),

        // .. required because this variant is non-exhaustive as well
        AdmitMoreVariants::VariantC { a, .. } => println!("it's a c"),

        // The wildcard match is required because more variants may be
        // added in the future
        _ => println!("it's a new variant")
    }
}