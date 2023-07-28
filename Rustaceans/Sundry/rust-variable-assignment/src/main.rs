/// .Text 段存放的是程序中的可执行代码
/// .Data 段保存的是已**初始化**了的全局变量和静态变量
/// .ROData (ReadOnlyData) 段存放程序中的常量值, 如字符串常量
/// .BSS 段存放的是**未初始化**的全局变量和静态变量, 程序执行前会先进行一遍初始化

/**
 * Rust has two different types of constants which can be declared in any scope including global. Both require explicit type annotation:
 *      const: An unchangeable value (the common case).
 *      static: A possibly mutable variable with 'static lifetime. The static lifetime is inferred and does not have to be specified. Accessing or modifying a mutable static variable is unsafe.
 */
const G_ARRAY: [i32;5] = [10;5];
const G_X: i32 = 100;
static G_Y: i32 = 200;

fn main() {
    let s: &str = ".rodata";  //字符串字面量, 位于ROData段
    println!("&str is at: {:p}", s);
    println!("&str is at: {:?} (using func as_ptr())", s.as_ptr());
    println!("G_ARRAY is at: {:p}", &G_ARRAY);
    println!("G_X is at: {:p}", &G_X);
    println!("G_Y is at: {:p}", &G_Y);

    let x_boxed = Box::new(10);
    println!("(HEAP) x_boxed is at: {:p}", x_boxed);
    
    let var = 10;
    let nums = [1, 2, 3, 4, 5, 6];
    let vec = vec![10, 20, 30, 40, 50, 60];
    println!("(STACK) var is at: {:p}", &var);
    println!("(STACK) nums is from: {:p}", &nums);
    println!("(STACK) vec ptr is at: {:p}", &vec);

    // String::from 将字符串字面量从内存中的代码区（ROData段）复制一份到堆, 并用栈上分配的变量指向堆内存
    let ss = String::from("hello");
    use std::mem::transmute;
    let pstr: [usize; 3] = unsafe { transmute(ss) };
    println!("pstr: 0x{:x}", pstr[0]);
}
