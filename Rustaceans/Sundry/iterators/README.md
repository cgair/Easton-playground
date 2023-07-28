# Iterator
[Iterator is a trait defined in the standard library.](https://doc.rust-lang.org/book/ch13-02-iterators.html#the-iterator-trait-and-the-next-method)
The definition of the trait looks like this:
```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```
Iterator trait requires that you also define an `Item` type, and this `Item` type is used in the return type of the next method. In other words, the `Item` type will be the type returned from the iterator.
Essentially, an iterator is a thing that allows you to traverse some sort of a sequence. Note that since Rust's iterators are **lazy**, this sequence could be generated on the fly - you could just as well traverse an existing array of finite length or create an iterator that keeps spewing out random numbers infinitely:
```rust
let inf_seq = (1..).into_iter();
```
> Laziness in programming is this general idea of delaying a computation until it's actually needed. A lazy iterator doesn't need to know all the elements it's going to return when it's first initialized - it can compute every next element when/if it's asked for.

## Iterator & IntoIterator
[Vec 本身并没有实现 Iterator](https://doc.rust-lang.org/std/vec/struct.Vec.html#), 也就是说, 你无法对 Vec 本身调用 .next() 方法. 但是, 查看 Vec 的 api 时发现 Vec 实现了 [IntoIterator 的 trait.](https://doc.rust-lang.org/src/alloc/vec/mod.rs.html#2762)
By implementing `IntoIterator` for a type, you define how it will be converted to an iterator.
```rust
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
}
```
- somehow 只需要记住, 只有实现了 `Iterator` 才能称为迭代器, 才能调用 next().
- `IntoIterator` 强调的是某一个类型如果实现了该 trait, 它可以通过 into_iter(), iter() 等方法变成一个迭代器.
  - One benefit of implementing `IntoIterator` is that your type will [work with Rust’s `for` loop syntax](https://doc.rust-lang.org/std/iter/index.html#for-loops-and-intoiterator).
