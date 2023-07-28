[The Rustonomicon](https://doc.rust-lang.org/nomicon/)
# 1. Meet Safe and Unsafe
Rust can be thought of as a combination of two programming languages: Safe Rust and Unsafe Rust.
## [Q - What's the relationship between Safe Rust and Unsafe Rust? How do they interact?]
unsafe keyword:
* declare the existence of contracts the compiler can't check
* declare that a programmer has checked that these contracts have been upheld.
Safe Rust 必须相信它所涉及的任何 Unsafe Rust 都已正确编写. 另一方面, Unsafe Rust 不能毫无顾忌地信任 Safe Rust.

// TODO

# 2. Data Layout
**Type Layout**
* [类型的 Layout](https://doc.rust-lang.org/reference/type-layout.html#type-layout) 是 its size, alignment, and the relative offsets of its fields. (大小, 对齐方式和字段的相对偏移量)
**Size and Alignment**
* 值的对齐方式指定了哪些地址可以有效地存储该值: 对齐为 2 的值必须存储在偶数地址, 而对齐为 1 的值可以存储在任何地址 (对齐以字节为单位, 必须至少为 1, 并且始终为 2 的幂). 使用 align_of_val 函数检查值的对齐情况.
* 值的大小始终是其对齐方式的倍数 (包括对齐填充). 使用 size_of_val 函数检查值的大小.
## [Q - How data is represented in Rust?]
1. `data_layout/main.rs`
2. 所有用户定义的复合类型 (structs, enums, and unions) 的指定该类型布局的 representation「**The representation of a type can be changed by applying the repr attribute (#[repr()]) to it**」:
   * Default: 默认情况下, 复合结构的对齐等于其字段对齐的最大值 (且会 **reorder**).
   * C: 为创建可与 C 语言互操作的类型.
   * The primitive representations
   * transparent


# 4. Type Conversions
归根结底, 一切都只是某处的一堆 bits, 而类型系统只是帮助我们正确使用这些 bits.
具有某种类型的 bits 有两个常见问题:
* 将这些 bits 重新解释为不同的类型;
* change the bits to have equivalent meaning for a different type.

## Casts
an exhaustive list of all the true casts and casting semantics on the reference.

NOTE:
* 1. Lengths when casting raw slices
* 2. 强制转换不具有**传递性**, 即使 e as U1 as U2 是一个有效的表达式, e as U2 也不一定如此.

## Transmutes
// TODO

# 6. Ownership Based Resource Management (OBRM)
> OBRM (AKA [RAII](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization): Resource Acquisition Is Initialization) is a programming idiom.

## Constructors
There is exactly one way to create an instance of a user-defined type: name it, and initialize all its fields at once.

## Destructors
What the language does provide is full-blown automatic destructors through the [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html) trait.

# 8. Concurrency
## Data Races and Race Conditions
data races are defined as:
* two or more threads concurrently accessing a location of memory
* one or more of them is a write
* one or more of them is unsynchronized

Data races 主要是通过 Rust 的 ownership system 来防止: 给可变引用起别名是不可能的 (it's impossible to alias a mutable reference).
Interior mutability 使这变得更加复杂, 但我们有 `Send and Sync`.

## Send and Sync
* 如果可以安全地将其发送到另一个线程, 则该类型为 `Send`
* 如果线程之间共享是安全的, 则类型为 `Sync` (当且仅当 &T 为 Send 时, T 为 Sync)

**例外:**
1. raw pointers are neither Send nor Sync (because they have no safety guards).
2. UnsafeCell isn't Sync (and therefore Cell and RefCell aren't).
3. Rc isn't Send or Sync (because the refcount is shared and unsynchronized).

## Atomics
### Compiler Reordering
Compilers fundamentally want to be able to do all sorts of complicated transformations to reduce data dependencies and eliminate dead code. 
In particular, they may radically change the actual order of events, or make events never occur!
```rust
let x = 1;
let y = 3;
let x = 2;
// The compiler may conclude that it would be best if your program did:
let x = 2;
let y = 3;
```

### Hardware Reordering
