// 问题：发布同步与 SeqCst 全序分别保证读者能观察到什么？
// 输入与状态：先写 payload=42 再 Release ready；另一个例子双方先 store 再 load。
// 结果证据：Acquire 读到 ready 后、join 前读取 42；SeqCst 排除双方都读 false。
// 改一处：把 ready 的 ordering 改成 Relaxed，观察正常运行也可能成功，但发布保证已经消失。
// 边界：一次运行不能穷尽弱内存行为；所有字段都为原子，弱化序不会变成非原子 data race。
// 运行：cargo run --locked --example 07-memory-ordering

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Barrier;
use std::thread;

fn published_payload() -> usize {
    let payload = AtomicUsize::new(0);
    let ready = AtomicBool::new(false);
    thread::scope(|scope| {
        scope.spawn(|| {
            payload.store(42, Ordering::Relaxed);
            ready.store(true, Ordering::Release);
        });
        while !ready.load(Ordering::Acquire) {
            thread::yield_now();
        }
        // This read and assertion precede scope's automatic join.
        let observed = payload.load(Ordering::Relaxed);
        assert_eq!(observed, 42);
        observed
    })
    // Both fields are atomic: weakening the flag ordering would lose the guarantee,
    // not introduce a non-atomic data race. A normal run need not expose a stale read.
}

fn seqcst_observations() -> (bool, bool) {
    let x = AtomicBool::new(false);
    let y = AtomicBool::new(false);
    let start = Barrier::new(2);
    thread::scope(|scope| {
        let left = scope.spawn(|| {
            start.wait();
            x.store(true, Ordering::SeqCst);
            y.load(Ordering::SeqCst)
        });
        let right = scope.spawn(|| {
            start.wait();
            y.store(true, Ordering::SeqCst);
            x.load(Ordering::SeqCst)
        });
        (left.join().unwrap(), right.join().unwrap())
    })
    // A single SeqCst order cannot place both loads before the other worker's store
    // while preserving each worker's store-before-load order.
}

pub fn main() {
    println!("P1: Acquire / Release / SeqCst");
    let published = published_payload();
    let (saw_y, saw_x) = seqcst_observations();
    assert!(saw_y || saw_x);
    println!("Acquire/Release payload before join: {published}; SeqCst forbids both false");
    // Assertions check this execution; they do not exhaust weak-memory behavior or prove liveness.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acquire_observes_the_payload_published_before_release() {
        assert_eq!(published_payload(), 42);
    }

    #[test]
    fn seqcst_forbids_both_loads_observing_the_initial_value() {
        let (saw_y, saw_x) = seqcst_observations();
        assert!(saw_y || saw_x);
    }
}
