use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Barrier, Mutex, RwLock};
use std::thread;

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

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

fn channel_messages() -> Vec<String> {
    let (sender, receiver) = mpsc::sync_channel(1);
    let producer = thread::spawn(move || {
        sender.send(String::from("one")).unwrap();
        sender.send(String::from("two")).unwrap();
    });
    // Drain before join: the producer may be waiting for capacity.
    // The last sender drops when the producer exits, so iteration can finish.
    let received = receiver.into_iter().collect();
    producer.join().unwrap();
    received
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

fn main() {
    println!("P0: Send / Sync");
    assert_send::<RefCell<u8>>(); // Whole-value transfer is different from shared access.
    assert_sync::<Mutex<RefCell<u8>>>();
    assert_send::<Arc<Mutex<RefCell<u8>>>>();
    // assert_sync::<RefCell<u8>>(); // E0277: RefCell is not Sync.
    // assert_send::<Arc<RefCell<u8>>>(); // Arc does not make its payload Sync.
    println!("RefCell<u8>: Send; Mutex<RefCell<u8>>: Sync");
    let shared = Arc::new(7);
    let worker_value = Arc::clone(&shared);
    let result = thread::spawn(move || *worker_value + 1).join().unwrap();
    assert_eq!(result, 8);
    println!("Arc moved across thread: {result}");

    println!("P0: Arc / Mutex / Channel / logical race");
    assert_eq!(mutex_count(4, 1000), 4000);
    println!(
        "Mutex count: 4000; owned messages: {:?}",
        channel_messages()
    );
    let lost = lost_update();
    assert_eq!(lost, 1);
    println!("Separate read/write locks: 2 increments -> {lost} (lost update)");
    // Waiting while holding one lock and acquiring another in opposite order can deadlock.
    // Mutex protects memory access; it does not validate a multi-step business transaction.

    println!("P1: RwLock / Atomic");
    let value = RwLock::new(7);
    {
        let left = value.read().unwrap();
        let right = value.read().unwrap();
        assert_eq!(*left + *right, 14);
    }
    *value.write().unwrap() = 8; // Release read guards before requesting a write lock.
    assert_eq!(*value.read().unwrap(), 8);
    assert_eq!(atomic_count(4, 1000), 4000);
    println!("RwLock after write: 8; Relaxed fetch_add after join: 4000");
    // These workers do not panic. In real code, poisoning calls for checking state invariants.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutex_and_atomic_preserve_all_increments() {
        for (workers, increments) in [(4, 1000), (0, 10), (2, 0), (1, 1)] {
            assert_eq!(mutex_count(workers, increments), workers * increments);
            assert_eq!(atomic_count(workers, increments), workers * increments);
        }
    }

    #[test]
    fn separate_locked_read_and_write_still_lose_an_update() {
        assert_eq!(lost_update(), 1);
        assert_eq!(mutex_count(2, 1), 2);
    }

    #[test]
    fn channel_transfers_messages_and_finishes_after_sender_drop() {
        assert_eq!(channel_messages(), ["one", "two"]);
    }

    #[test]
    fn disconnected_receiver_returns_the_undelivered_message() {
        let (sender, receiver) = mpsc::sync_channel(1);
        drop(receiver);
        assert_eq!(sender.send(String::from("unsent")).unwrap_err().0, "unsent");
    }
}
