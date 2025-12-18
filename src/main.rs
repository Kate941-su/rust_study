use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;


fn main() {
    println!("=== 1. Smart Pointer ===");
    
    println!("=== 1.1 Box ===");
    let x: i32 = 1; // on a stack
    let y: Box<i32> = Box::new(2);
    println!("x + y = {}", x + *y);

    println!("=== 1.2 Rc, Arc ===");
    let x= Rc::new(32);
    let y = x.clone();
    eprintln!("x = {0:p} (points to {0:})", x);
    eprintln!("y = {0:p} (points to {0:})", y);

    let counter = Arc::new(Mutex::new(0));
    let thread = thread::spawn({
        let counter = counter.clone();
        move || {
            for _ in 0..100000 {
                // カウンタのロックを取得
                let mut counter = counter.lock().unwrap();
                // 偶数なら1を足す
                if *counter % 2 == 0 {
                    *counter += 1;
                }
            }
        }
    });

    for _ in 0..100000 {
        // カウンタのロックを取得
        let mut counter = counter.lock().unwrap();
        // 奇数なら1を足す
        if *counter % 2 == 1 {
            *counter += 1;
        }
    }
    thread.join().unwrap();
    
    // カウンタの最終的な値を取得
    let counter = *counter.lock().unwrap();
    eprintln!("counter = {}", counter);
    

    println!("=== End Smart Pointer ===");
}
