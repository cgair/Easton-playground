# trait object
Use trait objects to get some object-oriented features in Rust. 动态分派可以为您的代码提供一些灵活性, 以换取一些运行时性能.
## Object-Oriented Programming Features of Rust. 
[Characteristics of Object-Oriented Languages](https://doc.rust-lang.org/book/ch17-01-what-is-oo.html)
多态性意味着如果多个对象共享某些特征, 可以在运行时相互替换多个对象.

Rust 提供了安全的抽象以避免产生安全问题或者错误.
* 比如我们使用 `RC` 而不直接管理引用计数, 使用 `Box` 而不是 `malloc/free` 直接管理内存分配.
* 同样, `dyn Trait` 隐藏了复杂而又为危险的虚表实现, 为我们提供了简单而又安全的动态分发.

## [Q - What options for polymorphism exist in Rust?]
There are two primary ways, and both of these have trade-offs to consider when it comes to performance as well as binary size:
* Static Dispatch - this approach leverages generics (also called parametric polymorphism) and (usually) trait bounds to provide the flexibility we need while still maintaining full type safety. This approach is extremely performant (in Rust this is known as a "zero-cost abstraction") - however, due to monomorphization (使用 trait bound 或者 impl trait 方式实现编译期单态化, 根据类型参数生成对应的结构或者函数), this does create a larger binary size.
* Dynamic Dispatch - this approach uses "trait objects" to punt the decision of which type is required to satisfy some kind of polymorphic interface to runtime. This cuts down on binary size (as no monomorphization is used here) but incurs a performance penalty due to the extra lookup at runtime (由于 trait object 是动态大小类型, 无法在编译期确定类型大小, 所以一般会使用指向 trait object 的引用或者指针来操作 trait object, 每次调用 trait object 的方法时, 需要解引用该胖指针). [This approach also explicitly forbids the use of generics](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#object-safety-is-required-for-trait-objects).
  * **动态分发:** 指向 trait object 的引用或者指针本质上是一个胖指针, 其中包含了指向擦除了具体类型的对象指针与虚函数表. 与静态分派相比, 动态分派不是根据使用的类型复制多态函数, 而是创建一个实现, 该实现在根据运行时计算的指针调用不同的底层类型和方法. 由于没有代码重复, 二进制文件的大小更小, 但由于需要进行额外的查找 (性能的影响很小).
  * [This gets us to the core of what a trait object actually is.](https://oswalt.dev/2021/06/polymorphism-in-rust/) In short, it’s a pointer. Specifically, it’s a pointer to portion of memory where a type’s bound methods can be found, as well as a few other things (like destructors). This is commonly referred to as a ["virtual method table"](https://en.wikipedia.org/wiki/Virtual_method_table), or "vtable".

## [Q -  Why `dynamic_dispatch` function is able to appear once in the compiled program?]

    Box<dyn Trait>                                              &dyn Trait
  +----------+----------+                                 +----------+----------+
  |   data   |  vtable  |                                 |   data   |  vtable  |
  +----|-----+----|-----+                                 +----|-----+----|-----+
       |          |                                            |          |
                  |                                                       |
                  |                                                       |
                  |                                                       |
                  |                                                       |
                  +-----------------------------------------+             |
                                                            |             |
                                                            |             |
                                                            |             |
                                                            V             V
                                                        +------------------------+
                                                        |       destructor       |
                                                        +------------------------+
                                                        |          size          |
                                                        +------------------------+
                                                        |         align          |
                                                        +------------------------+
                                                        |        method1         |
                                                        +------------------------+
                                                        |        method2         |
                                                        +------------------------+


```bash
objdump --disassemble=polymorphism::main -S -C target/debug/polymorphism -EL -M intel --insn-width=8
#
#fn main() {
#    7d00:       50                              push   rax
#    
#    dynamic_dispatch(&Lion{});
#    7d01:       48 8d 3d 68 53 03 00            lea    rdi,[rip+0x35368]        # 3d070 <_fini+0xbcc>
#    7d08:       48 8d 35 b9 44 04 00            lea    rsi,[rip+0x444b9]        # 4c1c8 ##<__do_global_dtors_aux_fini_array_entry+0x90>
#    7d0f:       e8 cc ff ff ff                  call   7ce0 <polymorphism::dynamic_dispatch>
#    dynamic_dispatch(&Tiger{});
#    7d14:       48 8d 3d 55 53 03 00            lea    rdi,[rip+0x35355]        # 3d070 <_fini+0xbcc>
#    7d1b:       48 8d 35 c6 44 04 00            lea    rsi,[rip+0x444c6]        # 4c1e8 #<__do_global_dtors_aux_fini_array_entry+0xb0>
#    7d22:       e8 b9 ff ff ff                  call   7ce0 <polymorphism::dynamic_dispatch>
#    dynamic_dispatch(&Bear{});
#    7d27:       48 8d 3d 42 53 03 00            lea    rdi,[rip+0x35342]        # 3d070 <_fini+0xbcc>
#    7d2e:       48 8d 35 d3 44 04 00            lea    rsi,[rip+0x444d3]        # 4c208 <__do_global_dtors_aux_fini_array_entry+0xd0>
#    7d35:       e8 a6 ff ff ff                  call   7ce0 <polymorphism::dynamic_dispatch>
#}
#    7d3a:       58                              pop    rax
#    7d3b:       c3                              ret    


#0000000000007ce0 <polymorphism::dynamic_dispatch>:
# fn dynamic_dispatch(t: &dyn Growler) {
#    7ce0:	48 83 ec 18             	sub    rsp,0x18
#    7ce4:	48 89 7c 24 08          	mov    QWORD PTR [rsp+0x8],rdi
#    7ce9:	48 89 74 24 10          	mov    QWORD PTR [rsp+0x10],rsi
#    t.growl();
#    7cee:	ff 56 18                	call   QWORD PTR [rsi+0x18]
#}
#    7cf1:	48 83 c4 18             	add    rsp,0x18
#    7cf5:	c3                      	ret    
#    7cf6:	66 2e 0f 1f 84 00 00 00 	nop    WORD PTR cs:[rax+rax*1+0x0]
#    7cfe:	00 00 
```
因为它 call a function at a **calculated location**, specifically an offset based on whatever value is in the `rsi` register.
So we know from the previous objdump that `rsi` will contain `0x4c1c8` when our `Lion` type is used, `0x4c1e8` for `Tiger`, and `0x4c208` for `Bear`. 

**And why is the program adding `0x18` to this value before calling the resulting memory location (line 31)?**

我们希望调用的方法实际上位于距每个`vtable`头 24 个字节的偏移处 (`the hexidecimal equivalent for this is 0x18`). 这就是为什么我们的 `dynamic_dispatch` 函数在调用指令之前将`0x18`添加到` rsi `. `rsi` 包含我们想要的类型的 vtable 的位置, 并将 `0x18` 添加到这个位置可以让我们找到想要访问的这个 `vtable` 中的条目.

在我们程序的整个生命周期中加载到 `rsi` 的三个 `memory location` 中的每一个都是不同类型的 vtable 所在的位置. 但是, vtable 中包含指向我们希望调用的方法的指针的部分实际上位于距每个表开头 24 个字节 (hexidecimal 0x18) 的偏移处. 这就是为什么我们的 dynamic_dispatch 函数在调用指令之前将它添加到 rsi 的原因. 
然而, 位于此处的仍然不是我们的方法, 而是另一个指针: **QWORD PTR [rsi+0x18] - 它首先加载位于 rsi + 0x18 的值作为指针, 然后调用由该指针表示的内存位置.**


# REFERENCE
[Trait object 是 Rust 动态分发(多态的需求)的实现方式](https://rustmagazine.github.io/rust_magazine_2021/chapter_8/ant-futures-compat.html)。
[Exploring Dynamic Dispatch in Rust](https://alschwalm.com/blog/static/2017/03/07/exploring-dynamic-dispatch-in-rust/)
