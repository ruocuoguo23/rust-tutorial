// 问题：每次读写都加锁，为什么两个自增仍然丢一个？
// 输入与状态：Barrier 让两个线程都先读到 0，再分别写回 old + 1。
// 结果证据：两次自增后确定得到 1；不存在不受同步保护的内存读写。
// 改一处：把读和写放进一个 guard，并把 Barrier 移到加锁之前，结果变为 2。
// 边界：safe Rust 阻止 data race，但不阻止业务逻辑竞态；持锁等待 Barrier 会死锁。
// 运行：cargo run --locked --example 05-logical-race

use std::sync::{Barrier, Mutex};
use std::thread;

fn lost_update() -> usize {
    let count = Mutex::new(0);
    let both_read = Barrier::new(2);
    thread::scope(|scope| {
        for _ in 0..2 {
            scope.spawn(|| {
                let old = *count.lock().unwrap(); // This guard ends at the semicolon.
                both_read.wait(); // Both workers read 0 before either writes: deterministic race.
                *count.lock().unwrap() = old + 1;
            });
        }
    });
    count.into_inner().unwrap()
}

pub fn main() {
    println!("P0: logical race");
    let lost = lost_update();
    assert_eq!(lost, 1);
    println!("Separate read/write locks: 2 increments -> {lost}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separate_locked_read_and_write_still_lose_an_update() {
        assert_eq!(lost_update(), 1);
    }
}
