// 问题：多个线程获取两把锁时，怎样避免相反顺序形成等待环？
// 输入与状态：两个 worker 都先锁 first，再锁 second，然后各自增加两个计数。
// 结果证据：两线程 join 完成，两个计数都是 2。
// 改一处：调换一个 worker 的加锁顺序，画出双方各持一锁的等待环，不运行故意死锁。
// 边界：示例不会运行故意死锁的代码；一次完成不能证明任意程序的活性。
// 运行：cargo run --locked --example 07-lock-order

use std::sync::Mutex;
use std::thread;

fn same_lock_order() -> (usize, usize) {
    let first = Mutex::new(0);
    let second = Mutex::new(0);
    thread::scope(|scope| {
        for _ in 0..2 {
            scope.spawn(|| {
                let mut first_guard = first.lock().unwrap();
                let mut second_guard = second.lock().unwrap();
                *first_guard += 1;
                *second_guard += 1;
            });
        }
        // Do not run this opposite-order worker alongside those above:
        // scope.spawn(|| {
        //     let _second_guard = second.lock().unwrap();
        //     let _first_guard = first.lock().unwrap();
        // });
        // If workers each hold one lock and wait for the other, they deadlock.
    });
    (first.into_inner().unwrap(), second.into_inner().unwrap())
}

pub fn main() {
    println!("P0: consistent lock order");
    let counts = same_lock_order();
    assert_eq!(counts, (2, 2));
    println!("Same lock order completes: {counts:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consistent_lock_order_allows_both_workers_to_complete() {
        assert_eq!(same_lock_order(), (2, 2));
    }
}
