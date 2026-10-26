// 问题：多个 reader 与独占 writer 如何分阶段访问同一值？
// 输入与状态：两个 reader 都完成后，writer 将 7 改为 8。
// 结果证据：读阶段为 [7, 7]，写阶段后为 8；输出与调度顺序无关。
// 改一处：增加一个 reader，观察共享读阶段仍得到 7。
// 边界：阶段安排为了稳定观察；不证明读写公平性、吞吐或无饥饿。
// 运行：cargo run --locked --example 05-rwlock

use std::sync::{Arc, RwLock};
use std::thread;

fn rwlock_phases() -> (Vec<u32>, u32) {
    let value = Arc::new(RwLock::new(7));
    let readers: Vec<_> = (0..2)
        .map(|_| {
            let value = Arc::clone(&value);
            thread::spawn(move || *value.read().unwrap())
        })
        .collect();
    let before = readers
        .into_iter()
        .map(|reader| reader.join().unwrap())
        .collect();
    // Finish the read phase before starting the writer; output is independent of scheduling.
    let writer_value = Arc::clone(&value);
    thread::spawn(move || *writer_value.write().unwrap() = 8)
        .join()
        .unwrap();
    let after = *value.read().unwrap();
    (before, after)
}

pub fn main() {
    println!("P1: RwLock");
    let (before, after) = rwlock_phases();
    assert_eq!(before, [7, 7]);
    assert_eq!(after, 8);
    println!("RwLock: {before:?} -> {after}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rwlock_read_phase_precedes_the_write_phase() {
        assert_eq!(rwlock_phases(), (vec![7, 7], 8));
    }
}
