// 问题：学过共享所有权与读写锁后，如何实现跨线程共享的 Store？
// 输入与状态：空 Store 经 writer 写入；join 后 reader 查询同一份表。
// 结果证据：language 为 Some("Rust")，missing 为 None；测试检查更新与多写入者。
// 改一处：保存 get 的结果后更新同一键，观察旧 String 不随表改变。
// 边界：这是综合应用；Send/Sync 不自动加锁，expect 不实现中毒恢复。
// 运行：cargo run --locked --example 07-shared-store

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
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

pub fn main() {
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
