// [green threads explained in 200 lines of rust](https://cfsamson.gitbook.io/green-threads-explained-in-200-lines-of-rust/)
//! An implementation of green threads
#![feature(naked_functions)]   // Marking the a function as #[naked]removes the prologue and epilogue.
use std::arch::asm;

const DEFAULT_STACK_SIZE: usize = 1024 * 1024 * 2;
const MAX_THREADS: usize = 4;
static mut RUNTIME: usize = 0;

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

struct Thread {
    /// each thread has an id so we can separate them from each other
    id: usize,
    stack: Vec<u8>,
    /// a context representing the data our CPU needs to resume where it left off on a stack
    ctx: ThreadContext,
    /// thread state
    state: State
}

#[derive(PartialEq, Eq, Debug)]
enum State {
    /// the thread is available and ready to be assigned a task if needed
    Available,
    /// thread is running
    Running,
    /// the thread is ready to move forward and resume execution
    Ready,
}

impl Thread {
    fn new(id: usize) -> Self {
        Self {
            id,
            stack: vec![0_u8; DEFAULT_STACK_SIZE],  //  That is not needed and is not an optimal use of our resources since we allocate memory for threads we might need instead of allocating on first use.
            ctx: ThreadContext::default(),
            state: State::Available
        }
    }
}

// Runtime is going to be our main entry point. 
// We are basically going to create a very small, simple runtime to schedule and switch between our threads. 
pub struct Runtime {
    threads: Vec<Thread>,
    /// indicate which thread we are currently running
    current: usize
}

impl Runtime {
    // When we instantiate our Runtime we set up a base thread.
    pub fn new() -> Self {
        // This will be our base thread, which will be initialized in 
        // the `running` state
        let base_thread = Thread {
            id: 0,
            stack: vec![0_u8; DEFAULT_STACK_SIZE],
            ctx: ThreadContext::default(),
            state: State::Running,
        };
        let mut threads = vec![base_thread];
        let mut available_threads: Vec<Thread> = (1..MAX_THREADS).map(|i| Thread::new(i)).collect();
        threads.append(&mut available_threads);

        Self {
            threads,
            current: 0
        }
    }
    /// This is cheating a bit, but we need a pointer to our Runtime 
    /// stored so we can call yield on it even if we don't have a 
    /// reference to it.
    pub fn init(&self) {
        unsafe {
            let r_ptr: *const Runtime = self;
            RUNTIME = r_ptr as usize;
        }
    }

    // This is where we start running our run-time. It will continually call t_yield() until it returns false which means that there is no more work to do and we can exit the process
    pub fn run(&mut self) -> ! {
        while self.t_yield() {}
        std::process::exit(0);
    }

    #[inline(never)]
    fn t_yield(&mut self) -> bool {
        // we go through all the threads and see if anyone is in the Ready state which indicates it has a task it is ready to make progress on
        let mut pos = self.current;
        while self.threads[pos].state != State::Ready {
            pos += 1;
            if pos == self.threads.len() {
                pos = 0;
            }
            if pos == self.current {
                return false;
            }
        }
        // If we find a thread that's ready to be run we change the state of the current thread from Running to Ready
        // Then we call switch which will save the current context (the old context) and load the new context into the CPU
        if self.threads[self.current].state != State::Available {
            self.threads[self.current].state = State::Ready;
        }

        self.threads[pos].state = State::Running;
        let old_pos = self.current;
        self.current = pos;

        unsafe {
            let old: *mut ThreadContext = &mut self.threads[old_pos].ctx;
            let new: *const ThreadContext = &self.threads[pos].ctx;
            asm!("call switch", in("rdi") old, in("rsi") new, clobber_abi("C"));
        }
        self.threads.len() > 0
    }

    pub fn spawn(&mut self, f: fn()) {
        let available = self.threads.iter_mut().find(|t| t.state == State::Available).expect("no available thread.");
        let size = available.stack.len();

        unsafe {
            let s_ptr = available.stack.as_mut_ptr().offset(size as isize);
            let s_ptr = (s_ptr as usize & !15) as *mut u8;
            std::ptr::write(s_ptr.offset(-16) as *mut u64, guard as u64);
            // std::ptr::write(s_ptr.offset(-24) as *mut u64, skip as u64);
            std::ptr::write(s_ptr.offset(-32) as *mut u64, f as u64);
            available.ctx.rsp = s_ptr.offset(-32) as u64;   // We want the f function to be the first to run so we set the base pointer to f and make sure it's 16 byte aligned
        }
        available.state = State::Ready;
    }

    // skip function which is there just to handle the gap when we return from f so that guard will get called on a 16 byte boundary
}

// guard function that will be called when the task we provide finishes and the function returns
fn guard() {

}

fn main() {
    let mut runtime = Runtime::new();
    runtime.init();
    runtime.spawn(|| {
        println!("THREAD 1 STARTING");
        let id = 1;
        for i in 0..10 {
            println!("thread: {} counter: {}", id, i);
            // yield_thread();
        }
        println!("THREAD 1 FINISHED");
    });
    runtime.spawn(|| {
        println!("THREAD 2 STARTING");
        let id = 2;
        for i in 0..15 {
            println!("thread: {} counter: {}", id, i);
            // yield_thread();
        }
        println!("THREAD 2 FINISHED");
    });
    runtime.run();
}
