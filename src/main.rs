use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;


pub struct Test {
    x: i32
}

impl Test {
    pub fn println(&self) {
        println!("{}", self.x)
    }
}

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


    let a = Rc::new(RefCell::new(Test{x : 11}));
    a.borrow().println();
    {
        let a1 = a.clone();
        let a2 = a.clone();
        a1.borrow_mut().x = 12;
        a2.borrow().println();
    }
    a.borrow_mut().x = 123;
    let a4 = a.borrow();
    let a5 = a.borrow();
    let a6 = a.borrow();
    println!("a4 = {:p}", &a4.x);
    println!("a5 = {:p}", &a5.x);
    println!("a6 = {:p}", &a6.x);
    println!("=== End Smart Pointer ===");

    println!("=== Paralell Smart Pointer ===");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            // やあ！立ち上げたスレッドから数字{}だよ！
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        // メインスレッドから数字{}だよ！
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // handle.join().unwrap();
    println!("=== End Paralell Smart Pointer ===");


}
