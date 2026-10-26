// 问题：原子 load/store 与原子 read-modify-write 有什么区别？
// 输入与状态：fetch_add 对同一计数器累加；Barrier 让两个 SeqCst load 先读到 0 再写 1。
// 结果证据：Relaxed fetch_add 得到 4000；SeqCst load + store 仍确定丢更新得到 1。
// 改一处：将丢更新中的 load/store 换成 fetch_add，观察计数变为 2。
// 边界：更强内存序不会把多个操作合并成事务；此计数器不用于发布其他数据。
// 运行：cargo run --locked --example 05-atomic-updates

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Barrier;
use std::thread;

fn atomic_count(workers: usize, increments: usize) -> usize {
    let count = AtomicUsize::new(0);
    thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| {
                for _ in 0..increments {
                    count.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    }); // Join completes all increments; this counter does not publish another payload.
    count.load(Ordering::Relaxed)
}

fn seqcst_lost_update() -> usize {
    let count = AtomicUsize::new(0);
    let both_read = Barrier::new(2);
    thread::scope(|scope| {
        for _ in 0..2 {
            scope.spawn(|| {
                let old = count.load(Ordering::SeqCst);
                both_read.wait();
                count.store(old + 1, Ordering::SeqCst);
            });
        }
    });
    // Strong ordering does not merge load + store into one read-modify-write operation.
    count.load(Ordering::SeqCst)
}

pub fn main() {
    println!("P1: Atomic read-modify-write");
    assert_eq!(atomic_count(4, 1000), 4000);
    assert_eq!(seqcst_lost_update(), 1);
    println!("Relaxed fetch_add: 4000; SeqCst load/store: 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atomic_preserves_all_increments() {
        for (workers, increments) in [(4, 1000), (0, 10), (2, 0), (1, 1)] {
            assert_eq!(atomic_count(workers, increments), workers * increments);
        }
    }

    #[test]
    fn seqcst_does_not_make_separate_load_and_store_a_transaction() {
        assert_eq!(seqcst_lost_update(), 1);
        assert_eq!(atomic_count(2, 1), 2);
    }
}
