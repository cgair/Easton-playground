use std::fs;
use std::io::Write;
use std::arch::asm;
use std::path::Path;

const SSIZE: isize = 1024;
static mut S_PTR: *const u8 = 0 as *const u8;

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    rbx: u64,
    rbp: u64,
}

fn print_stack<P: AsRef<Path>>(filename: P) {
    let mut f = fs::File::create(filename).expect("create file");
    unsafe {
        for i in (0..SSIZE).rev() {
            writeln!(
                f,
                "mem: {:p}, val: {}",
                S_PTR.offset(i as isize),
                *S_PTR.offset(i as isize)
            )
            .expect("writing to file.");
        }
    }
}

fn hello() {
    println!("I LOVE WAKING UP ON A NEW STACK!");
    print_stack("AFTER.txt");

    loop {}
}

unsafe fn gt_switch(new: *const ThreadContext) {
    asm!(
        "mov rsp, [{0} + 0x00]",
        "ret",
        in(reg) new
    );
}

pub fn main() {
    let mut ctx = ThreadContext::default();
    let mut stack = vec![0_u8; SSIZE as usize];
    let stack_ptr = stack.as_mut_ptr();

    unsafe {
        let stack_bottom = stack.as_mut_ptr().offset(SSIZE);
        let sb_aligned = (stack_bottom as usize & !15) as *mut u8;
        S_PTR = sb_aligned;
        std::ptr::write(stack_ptr.offset(SSIZE - 16) as *mut u64, hello as u64);
        print_stack("BEFORE.txt");
        ctx.rsp = stack_ptr.offset(SSIZE - 16) as u64;
        gt_switch(&mut ctx);
    }
}