use std::sync::{
    atomic::{AtomicUsize, Ordering},
};
use std::thread;

// 0: lock is available, 1: lock is held
static SPINLOCK: SpinLock = SpinLock(AtomicUsize::new(0));
#[allow(non_upper_case_globals)]
static mut counter: isize = 0;

struct SpinLock(AtomicUsize); 

fn lock(lock: &SpinLock) {
    while lock.0.compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst) == Err(1) {
        // spin-wait (do nothing)
    }
    /* HOW TO AVOID SPINNING? */
}

fn un_lock(lock: &SpinLock) {
    lock.0.store(0, Ordering::Relaxed);
}

fn main() {
    let t1 = thread::spawn(move || {
        lock(&SPINLOCK);
        unsafe {
            for _ in 0..1000 { counter += 1; }
        }
        un_lock(&SPINLOCK);
    });

    let t2 = thread::spawn(move || {
        lock(&SPINLOCK);
        unsafe {
            for _ in 0..1000 { counter += 1; }
        }
        un_lock(&SPINLOCK);
    });
    t1.join().unwrap();
    t2.join().unwrap();
    unsafe { println!("main: end with counter = {}", counter); }
}