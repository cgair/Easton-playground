// [repr(Rust)](https://doc.rust-lang.org/nomicon/repr-rust.html)
// [Representations](https://doc.rust-lang.org/reference/type-layout.html#representations)

// /////////// //
//  Alignment  //
// /////////// //
// <https://en.wikipedia.org/wiki/Data_structure_alignment>
// <https://stackoverflow.com/questions/119123/why-isnt-sizeof-for-a-struct-equal-to-the-sum-of-sizeof-of-each-member/119134#119134>

/**
 * [Data Structure Alignment : How data is arranged and accessed in Computer Memory?]
 * [Data alignment] : Data alignment means putting the data in memory at an address equal to some multiple of the word size. 
 *                    This increases the performance of the system due to the way the CPU handles memory. 
 * [Data Structure Padding] : Now, to align the data, it may be necessary to insert some extra bytes between the end of the last data structure 
 *                            and the start of the next data structure as the data is placed in memory as multiples of fixed word size. 
 *                            This insertion of extra bytes of memory to align the data is called data structure padding.
 */

/*
Rust gives you the following ways to lay out composite data:
    structs (named product types)
    tuples (anonymous product types)
    arrays (homogeneous product types)
    enums (named sum types -- tagged unions)
    unions (untagged unions)

All user-defined composite types (structs, enums, and unions) have a representation that specifies what the layout is for the type. 
The possible representations for a type are:
    Default
    C
    The primitive representations
    transparent
*/

// //////////////////////// //
//         Default          //
// //////////////////////// //
// <https://doc.rust-lang.org/reference/type-layout.html#the-default-representation>

// By default, 
// 1. Composite structures have an alignment equal to the maximum of their fields' alignments.
struct A {
    a: u8,
    b: u32,
    c: u16
}

// 2. Rust does not lays out the fields in the order specified (Reorder)
struct Foo<T, U> {
    count: u16,
    data1: T,
    data2: U
}

enum Bar {
    A(u32),
    B(u64),
    C(u8)
}
/* might be laid out as:
    struct BarRepr {
        data: u64, // this is either a u64, u32, or u8 based on `tag`
        tag: u8,   // 0 = A, 1 = B, 2 = C
    }
    an enum consisting of a single outer unit variant (e.g. None) and a (potentially nested) non-nullable pointer variant (e.g. Some(&T)) makes the tag unnecessary. 
    A null pointer can safely be interpreted as the unit (None) variant. 
    The net result is that, for example, size_of::<Option<&T>>() == size_of::<&T>().
*/

#[test]
fn test_default_representation() {
    // 1
    let a = A { a: 1, b: 2, c: 3 };
    println!("A.a: {:p}, A.b: {:p}, A.c: {:p}", &a.a, &a.b, &a.c);
    /*
    A.a: 0x70000a697866, A.b: 0x70000a697860, A.c: 0x70000a697864
    struct A {
        b: u32,
        c: u16,
        a: u8,
        _pad: [u8; 1], // to align `b`
    }
    */
    
    // 2
    let foo = Foo::<u16, u32>{ count: 1, data1: 2, data2: 3};
    println!("Foo.a: {:p}, Foo.b: {:p}, Foo.c: {:p}", &foo.count, &foo.data1, &foo.data2);
    /*
    Foo.a: 0x700002fac864, Foo.b: 0x700002fac866, Foo.c: 0x700002fac860
    **if Rust didn't reorder fields, we would expect it to produce the following (quite simply wastes space)**
    struct Foo<u32, u16> {
        count: u16,
        _pad1: u16,
        data1: u32,
        data2: u16,
        _pad2: u16,
    }   quite simply wastes space
    */

    let bar_a = Bar::A(1);
    let bar_b = Bar::B(2);
    let bar_c = Bar::C(3);
    println!("bar_a: {:p}, bar_b: {:p}, bar_c: {:p}", &bar_a, &bar_b, &bar_c);

    use core::mem::size_of;
    println!("{} != {}", size_of::<Option<u64>>(), size_of::<u64>());
    println!("{} == {}", size_of::<Option<&u64>>(), size_of::<&u64>());
}


// //////////////////////// //
//   The C Representation   //
// //////////////////////// //
// <https://doc.rust-lang.org/reference/type-layout.html#the-c-representation>

// This representation can be applied to structs, unions, and enums. 
// The exception is zero-variant enums for which the C representation is an error.
// bindgen ./nodes.h -o binding.rs  --no-layout-tests

// #[repr(C)] Unions
#[repr(C)]
union Union {
    f1: u16,
    f2: [u8; 4],
}

#[repr(C)]
union SizeRoundedUp {
   a: u32,
   b: [u16; 3],
}

// #[repr(C)] Field-less Enums

// #[repr(C)] Enums With Fields
// The representation of a repr(C) enum with fields is a repr(C) struct with two fields.
// This Enum has the same representation as ...
#[repr(C)]
enum MyEnum {
    A(u32),
    B(f32, u64),
    C { x: u32, y: u8 },
    D,
}
// ... this struct.
#[repr(C)]
struct MyEnumRepr {
    tag: MyEnumDiscriminant,
    payload: MyEnumFields,
}

// This is the discriminant enum.
#[repr(C)]
enum MyEnumDiscriminant { A, B, C, D }

// This is the variant union.
#[repr(C)]
union MyEnumFields {
    A: MyAFields,
    B: MyBFields,
    C: MyCFields,
    D: MyDFields,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct MyAFields(u32);

#[repr(C)]
#[derive(Copy, Clone)]
struct MyBFields(f32, u64);

#[repr(C)]
#[derive(Copy, Clone)]
struct MyCFields { x: u32, y: u8 }

// This struct could be omitted (it is a zero-sized type), and it must be in
// C/C++ headers.
#[repr(C)]
#[derive(Copy, Clone)]
struct MyDFields;

#[test]
fn test_c_representation() {
    assert_eq!(std::mem::size_of::<Union>(), 4);  // From f2
    assert_eq!(std::mem::align_of::<Union>(), 2); // From f1

    assert_eq!(std::mem::size_of::<SizeRoundedUp>(), 8);  // Size of 6 from b,
                                                          // rounded up to 8 from
                                                      
    assert_eq!(std::mem::align_of::<SizeRoundedUp>(), 4); // alignment of a.
                                                          // From a
}


// //////////////////////// //
// Primitive Representation //
// //////////////////////// //
// <https://doc.rust-lang.org/reference/type-layout.html#primitive-representations>


// //////////////////////// //
//       transparent        //
// //////////////////////// //
// <https://doc.rust-lang.org/reference/type-layout.html#the-transparent-representation>

struct WithVec {
    a: u32,
    b: Vec<u8>
}

#[test]
fn test_another() {
    let with_vec = WithVec {
        a: 1,
        b: vec![2, 3, 4, 5]
    };
    println!("with_vec.a: {:p}, with_vec.b: {:p}", &with_vec.a, &with_vec.b);
    // let b_vec = with_vec.b;
    // let b = b_vec.as_ptr() as *const u8;

    let ptr = &&with_vec.b as *const _ as *const *const u8;
    
    unsafe{
        println!("           start: {:p}", ptr);
        println!("   offset 1 byte: {:p}", ptr.byte_offset(-1));
        println!("   offset 2 byte: {:p}", ptr.byte_offset(-2));
        println!("   offset 3 byte: {:p}", ptr.byte_offset(-3));
        println!("   offset 4 byte: {:p}", ptr.byte_offset(-4));
        println!("   offset 5 byte: {:p}", ptr.byte_offset(-5));
        println!("   offset 6 byte: {:p}", ptr.byte_offset(-6));
        println!("   offset 7 byte: {:p}", ptr.byte_offset(-7));
        println!("   offset 8 byte: {:p}", ptr.byte_offset(-8));
        println!("   offset 9 byte: {}", ptr.byte_offset(9).read() as usize);

        // println!("&b[0]: {:p}, b[0] = {}", b, *b );
        // println!("&b[1]: {:p}, b[1] = {}", b.offset(1), *b.offset(1));
        // println!("&b[2]: {:p}, b[2] = {}", b.offset(2), *b.offset(2));
        // println!("&b[3]: {:p}, b[3] = {}", b.offset(3), *b.offset(3));
    }
}