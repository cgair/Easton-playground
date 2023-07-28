/**
 * In this example we will create our own stack 
 * and make our CPU return out of it's current execution context 
 * and over to the stack we just created. 
 * Stack alignment on x86-64 is 16 bytes. We need to make sure to put our stack pointer to an address which is a multiple of 16
 */

// We need to use the Rust Nightly since we will use some features that are not stabilized yet
use core::arch::asm;
const SSIZE: isize = 48;    // set a small stack size here, only 48 bytes

#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp: u64,
}

/*
It means the function never returns (usually because it unconditionally panics or otherwise ends the program, or because it contains an infinite loop that prevents a return from ever happening).
The appendix describes it as:
    ! Always empty bottom type for diverging functions where "diverging" means "never returns".
*/
fn hello() -> ! {
    println!("I LOVE WAKING UP ON A NEW STACK!");

    loop {}
}

// we switch over to our own stack
unsafe fn gt_switch(new: *const ThreadContext) {
    asm!(
        "mov rsp, [{0} + 0x00]",    // The [] basically means: "get what's at this memory location", you can think of it as the same as dereferencing a pointer. 
        "ret",
        in(reg) new,
    );
}

fn main() {
    let mut ctx = ThreadContext::default();
    let mut stack = vec![0u8; SSIZE as usize];
    println!("   stack top: {:p}", &stack[0]);
    
    unsafe {
        let stack_bottom = stack.as_mut_ptr().offset(SSIZE);
        println!("stack bottom: {:p}", stack_bottom);
        let sb_aligned = (stack_bottom as usize & !15) as *mut u8;
        // std::ptr::write(sb_aligned.offset(-16) as *mut u64, 256 as u64);
        // std::ptr::write(sb_aligned.offset(-16) as *mut u64, 65536 as u64);
        // println!("{}", hello as u64);
        std::ptr::write(sb_aligned.offset(-16) as *mut u64, hello as u64);  // you have to bear in mind that the order the CPU writes an u64 as u8 bytes is dependent on it's endianness.
        for i in (0..SSIZE).rev() {
            println!("mem: {:p}, val: {}",
            sb_aligned.offset(i as isize),
            *sb_aligned.offset(i as isize))
        }
        ctx.rsp = sb_aligned.offset(-16) as u64;
        // println!("{}", *(ctx.rsp as *const u64));
        gt_switch(&mut ctx);
    }
}
