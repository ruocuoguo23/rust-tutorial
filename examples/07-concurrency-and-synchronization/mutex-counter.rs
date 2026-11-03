// 问题：两个线程怎样更新同一个计数，而不是各自的副本？
// 输入与状态：两个 Arc owner 共享 Mutex<usize>，每个线程加一次。
// 结果证据：join 等待两个更新完成，最终 count 为 2。
// 改一处：增加一个 worker，观察结果随已完成的更新数增加。
// 边界：Arc 共享存活责任；Mutex 保护这次读改写，不证明具体调度顺序。
// 运行：cargo run --locked --example 07-mutex-counter

use std::sync::{Arc, Mutex};
use std::thread;

pub fn main() {
    let count = Arc::new(Mutex::new(0_usize));
    let mut workers = Vec::new();
    for _ in 0..2 {
        let count = Arc::clone(&count);
        workers.push(thread::spawn(move || {
            *count.lock().unwrap() += 1;
        }));
    }
    for worker in workers {
        worker.join().unwrap();
    }
    assert_eq!(*count.lock().unwrap(), 2);
    println!("count = {}", *count.lock().unwrap());
}
