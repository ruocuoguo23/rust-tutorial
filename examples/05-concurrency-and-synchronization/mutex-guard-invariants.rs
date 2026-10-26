// 问题：锁必须覆盖哪些操作，guard 在什么时候解锁？
// 输入与状态：4 个线程各加 1000 次；guard 写入 8；两个线程转移余额 (10, 0)。
// 结果证据：计数为 4000，guard 退出后能再加锁，余额为 (8, 2) 且总额保持 10。
// 改一处：把余额减和加放进两个独立的 guard，检查中间状态是否仍保持总额。
// 边界：Mutex 保证持锁期间互斥；poisoning 后仍需检查业务状态不变量。
// 运行：cargo run --locked --example 05-mutex-guard-invariants

use std::sync::{Arc, Mutex, TryLockError};
use std::thread;

fn mutex_count(workers: usize, increments: usize) -> usize {
    let count = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..workers)
        .map(|_| {
            let count = Arc::clone(&count);
            thread::spawn(move || {
                for _ in 0..increments {
                    *count.lock().unwrap() += 1; // Read-modify-write uses one guard.
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    let result = *count.lock().unwrap();
    result
}

fn guard_lifetime() -> u32 {
    let value = Mutex::new(7);
    {
        let mut guard = value.lock().unwrap();
        *guard = 8;
        assert!(matches!(value.try_lock(), Err(TryLockError::WouldBlock)));
    } // RAII: Resource Acquisition Is Initialization; dropping the guard unlocks.
    let result = *value.try_lock().unwrap();
    result
}

fn transfer_balances() -> (u32, u32) {
    // A teaching model: each worker transfers one unit; the total must remain 10.
    let balances = Arc::new(Mutex::new((10, 0)));
    let handles: Vec<_> = (0..2)
        .map(|_| {
            let balances = Arc::clone(&balances);
            thread::spawn(move || {
                let mut guard = balances.lock().unwrap();
                guard.0 -= 1;
                guard.1 += 1;
                assert_eq!(guard.0 + guard.1, 10);
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    let result = *balances.lock().unwrap();
    result
}

pub fn main() {
    println!("P0: Mutex guard / invariants");
    assert_eq!(mutex_count(4, 1000), 4000);
    assert_eq!(guard_lifetime(), 8);
    let balances = transfer_balances();
    assert_eq!(balances, (8, 2));
    println!("Mutex count: 4000; guard drop unlocks; balances: {balances:?}");
    // These workers do not panic. In real code, poisoning calls for checking state invariants.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutex_preserves_all_increments() {
        for (workers, increments) in [(4, 1000), (0, 10), (2, 0), (1, 1)] {
            assert_eq!(mutex_count(workers, increments), workers * increments);
        }
        // Contrast with logical-race.rs: one guard covers the whole increment.
        assert_eq!(mutex_count(2, 1), 2);
    }

    #[test]
    fn dropping_guard_releases_the_lock_and_preserves_the_write() {
        assert_eq!(guard_lifetime(), 8);
    }

    #[test]
    fn one_guard_protects_the_balance_transfer() {
        let (available, reserved) = transfer_balances();
        assert_eq!((available, reserved), (8, 2));
        assert_eq!(available + reserved, 10);
    }
}
