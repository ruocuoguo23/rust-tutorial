// 问题：Send／Sync 在接口中怎样使用，跨线程共享的 Store 怎样实际读写？
// 输入与状态：保留类型边界检查；空 Store 经 writer 写入，join 后 reader 查询同一份数据。
// 结果证据：类型约束通过，旧示例得到 8；Store 输出 language = Some("Rust")，缺失键为 None。
// 改一处：取消 Arc<RefCell<u8>> 的 assert_send 注释，观察 E0277；move 不会补齐 Send。
// 边界：约束不自动加锁；Arc 不复制表；get 返回独立 String；expect 不实现锁中毒恢复。
// 运行：cargo run --locked --example 05-send-sync

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

// 接口声明线程安全能力；实际同步由实现的字段提供。
trait Store: Send + Sync {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&self, key: String, value: String);
}

#[derive(Default)]
struct MemoryStore {
    data: RwLock<HashMap<String, String>>,
}

impl Store for MemoryStore {
    fn get(&self, key: &str) -> Option<String> {
        let data = self.data.read().expect("store read lock poisoned");
        data.get(key).cloned()
    }

    fn set(&self, key: String, value: String) {
        let mut data = self.data.write().expect("store write lock poisoned");
        data.insert(key, value);
    }
}

fn assert_send<T: Send>() {}

fn assert_sync<T: Sync>() {}

pub fn main() {
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

    assert_send::<MemoryStore>();
    assert_sync::<MemoryStore>();
    let store: Arc<dyn Store> = Arc::new(MemoryStore::default());
    let writer_store = Arc::clone(&store);
    let writer = thread::spawn(move || {
        writer_store.set("language".to_owned(), "Rust".to_owned());
    });
    writer.join().expect("writer panicked");

    let reader_store = Arc::clone(&store);
    let reader = thread::spawn(move || {
        let value = reader_store.get("language");
        assert_eq!(value.as_deref(), Some("Rust"));
        println!("language = {value:?}");
    });
    reader.join().expect("reader panicked");
    assert_eq!(store.get("missing"), None);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_key_returns_none() {
        let store = MemoryStore::default();
        assert_eq!(store.get("missing"), None);
    }

    #[test]
    fn updating_a_key_preserves_the_previous_owned_result() {
        let store = MemoryStore::default();
        store.set("language".to_owned(), "Rust".to_owned());
        let previous = store.get("language").unwrap();
        store.set("language".to_owned(), "Go".to_owned());
        assert_eq!(previous, "Rust");
        assert_eq!(store.get("language").as_deref(), Some("Go"));
    }

    #[test]
    fn two_writers_update_the_same_store_without_losing_distinct_keys() {
        let store: Arc<dyn Store> = Arc::new(MemoryStore::default());
        let workers: Vec<_> = ["first", "second"]
            .into_iter()
            .map(|key| {
                let shared = Arc::clone(&store);
                thread::spawn(move || shared.set(key.to_owned(), key.to_owned()))
            })
            .collect();
        for worker in workers {
            worker.join().expect("writer panicked");
        }
        assert_eq!(store.get("first").as_deref(), Some("first"));
        assert_eq!(store.get("second").as_deref(), Some("second"));
    }
}
