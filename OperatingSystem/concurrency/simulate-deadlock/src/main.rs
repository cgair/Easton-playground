use lazy_static::lazy_static;
use std::{
    thread,
    time,
    sync::Mutex
};
lazy_static! {
    static ref MUTEX_A: Mutex<i32> = Mutex::new(0);
    static ref MUTEX_B: Mutex<i32> = Mutex::new(1);
}

fn main() {
    let mut handles = vec![];

    let t_a = thread::Builder::new()
        .name("thread_a".to_string())
        .spawn(|| {
            println!("Thread A waiting get Resource A");
            let _guard = MUTEX_A.lock().unwrap();
            println!("Thread A got Resource A");
            
            thread::sleep(time::Duration::from_secs(1));

            println!("Thread A waiting get Resource B");
            let _guard = MUTEX_B.lock().unwrap();
            println!("Thread A got Resource B");
        }).expect("thread spwan success");
    handles.push(t_a);

    let t_b = thread::Builder::new()
    .name("thread_b".to_string())
    .spawn(|| {
        println!("Thread B waiting get Resource B");
        let _guard = MUTEX_B.lock().unwrap();
        println!("Thread B got Resource B");
        
        thread::sleep(time::Duration::from_secs(1));

        println!("Thread B waiting get Resource A");
        let _guard = MUTEX_A.lock().unwrap();
        println!("Thread B got Resource A");
    }).expect("thread spwan success");
    handles.push(t_b);

    for h in handles {
        h.join().unwrap();
    }

    println!("This is not printed");
}
