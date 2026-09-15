use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Barrier, Mutex, RwLock, TryLockError};
use std::thread;

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

fn owned_thread_len(message: String) -> usize {
    // An owned String satisfies 'static without living for the whole program.
    thread::spawn(move || message.len()).join().unwrap()
}

fn scoped_sum(values: &[u32]) -> u32 {
    // scope waits for its workers, so borrowing the caller's local data is valid.
    // Counterexample: move copies the reference, not the referenced String.
    // let text = String::from("local");
    // let borrowed = &text;
    // thread::spawn(move || borrowed.len()).join().unwrap(); // E0597: text is not 'static.
    thread::scope(|scope| scope.spawn(|| values.iter().sum()).join().unwrap())
}

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

fn channel_messages() -> Vec<String> {
    let (sender, receiver) = mpsc::sync_channel(1);
    let producer = thread::spawn(move || {
        let message = String::from("one");
        sender.send(message).unwrap();
        // println!("{message}"); // E0382: send moved this non-Copy value.
        sender.send(String::from("two")).unwrap();
    });
    // Drain before join: the producer may be waiting for capacity.
    // The last sender drops when the producer exits, so iteration can finish.
    let received = receiver.into_iter().collect();
    producer.join().unwrap();
    received
}

fn undelivered_message() -> String {
    let (sender, receiver) = mpsc::sync_channel(1);
    drop(receiver);
    sender.send(String::from("unsent")).unwrap_err().0
}

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

fn main() {
    println!("P0: spawn / move / scope / join");
    let length = owned_thread_len(String::from("hello"));
    let values = vec![1, 2, 3];
    let sum = scoped_sum(&values);
    assert_eq!((length, sum), (5, 6));
    assert_eq!(values, [1, 2, 3]);
    println!("Moved String length: {length}; borrowed sum: {sum}; owner still usable");

    println!("P0: Send / Sync");
    assert_send::<RefCell<u8>>(); // Whole-value transfer is different from shared access.
    assert_sync::<Mutex<RefCell<u8>>>();
    assert_send::<&Mutex<RefCell<u8>>>(); // T: Sync iff &T: Send.
    assert_send::<Arc<Mutex<RefCell<u8>>>>();
    assert_send::<Arc<u8>>();
    assert_sync::<Arc<u8>>();
    // assert_send::<std::rc::Rc<u8>>(); // E0277: Rc's count is not atomic.
    // assert_sync::<RefCell<u8>>(); // E0277: RefCell is not Sync.
    // assert_send::<Arc<RefCell<u8>>>(); // E0277: Arc does not make its payload Sync.
    println!("RefCell<u8>: Send; Mutex<RefCell<u8>>: Sync");
    let shared = Arc::new(7);
    let worker_value = Arc::clone(&shared);
    let result = thread::spawn(move || *worker_value + 1).join().unwrap();
    assert_eq!(result, 8);
    println!("Arc moved across thread: {result}");

    println!("P0: Arc / Mutex / Channel / logical race");
    assert_eq!(mutex_count(4, 1000), 4000);
    assert_eq!(guard_lifetime(), 8);
    let balances = transfer_balances();
    assert_eq!(balances, (8, 2));
    println!("Mutex count: 4000; guard drop unlocks; balances: {balances:?}");
    let messages = channel_messages();
    let returned = undelivered_message();
    assert_eq!(messages, ["one", "two"]);
    assert_eq!(returned, "unsent");
    println!("Owned messages: {messages:?}; disconnected send returns: {returned}");
    let lost = lost_update();
    assert_eq!(lost, 1);
    assert_eq!(same_lock_order(), (2, 2));
    println!("Separate read/write locks: 2 increments -> {lost}; same lock order completes");

    println!("P1: RwLock / Atomic / Ordering");
    let (before, after) = rwlock_phases();
    assert_eq!(before, [7, 7]);
    assert_eq!(after, 8);
    assert_eq!(atomic_count(4, 1000), 4000);
    assert_eq!(seqcst_lost_update(), 1);
    println!("RwLock: {before:?} -> {after}; Relaxed fetch_add: 4000; SeqCst load/store: 1");
    let published = published_payload();
    let (saw_y, saw_x) = seqcst_observations();
    assert!(saw_y || saw_x);
    println!("Acquire/Release payload before join: {published}; SeqCst forbids both false");
    // These workers do not panic. In real code, poisoning calls for checking state invariants.
    // Assertions check this execution; they do not exhaust weak-memory behavior or prove liveness.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_values_can_be_processed_and_joined() {
        assert_eq!(owned_thread_len(String::from("hello")), 5);
        assert_eq!(owned_thread_len(String::new()), 0);
    }

    #[test]
    fn scoped_threads_borrow_without_consuming_the_owner() {
        let values = vec![1, 2, 3];
        assert_eq!(scoped_sum(&values), 6);
        assert_eq!(values, [1, 2, 3]);
        assert_eq!(scoped_sum(&[]), 0);
    }

    #[test]
    fn mutex_and_atomic_preserve_all_increments() {
        for (workers, increments) in [(4, 1000), (0, 10), (2, 0), (1, 1)] {
            assert_eq!(mutex_count(workers, increments), workers * increments);
            assert_eq!(atomic_count(workers, increments), workers * increments);
        }
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

    #[test]
    fn separate_locked_read_and_write_still_lose_an_update() {
        assert_eq!(lost_update(), 1);
        assert_eq!(mutex_count(2, 1), 2);
    }

    #[test]
    fn consistent_lock_order_allows_both_workers_to_complete() {
        assert_eq!(same_lock_order(), (2, 2));
    }

    #[test]
    fn channel_transfers_messages_and_finishes_after_sender_drop() {
        assert_eq!(channel_messages(), ["one", "two"]);
    }

    #[test]
    fn disconnected_receiver_returns_the_undelivered_message() {
        assert_eq!(undelivered_message(), "unsent");
    }

    #[test]
    fn rwlock_read_phase_precedes_the_write_phase() {
        assert_eq!(rwlock_phases(), (vec![7, 7], 8));
    }

    #[test]
    fn seqcst_does_not_make_separate_load_and_store_a_transaction() {
        assert_eq!(seqcst_lost_update(), 1);
        assert_eq!(atomic_count(2, 1), 2);
    }

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
