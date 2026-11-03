// 问题：哪些值可以移入线程，哪些类型允许跨线程共享引用？
// 输入与状态：类型约束检查 String/Arc<u8>；worker 移入 Arc 并读到 7。
// 结果证据：编译通过并返回 8；类型边界测试保留 RefCell/Mutex 的对照。
// 改一处：取消 Arc<RefCell<u8>> 的 assert_send 注释，观察 E0277。
// 边界：Send/Sync 是能力要求；move 不会补齐它们，也不自动同步业务状态。
// 运行：cargo run --locked --example 07-send-sync

use std::sync::Arc;
use std::thread;

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

pub fn main() {
    assert_send::<String>();
    assert_send::<Arc<u8>>();
    assert_sync::<Arc<u8>>();
    // assert_send::<std::rc::Rc<u8>>(); // E0277：Rc 不是 Send。
    // assert_sync::<std::cell::RefCell<u8>>(); // E0277：RefCell 不是 Sync。
    // assert_send::<Arc<std::cell::RefCell<u8>>>(); // E0277：Arc 不会补齐内部 Sync。

    let shared = Arc::new(7_u8);
    let worker_value = Arc::clone(&shared);
    let result = thread::spawn(move || *worker_value + 1).join().unwrap();
    assert_eq!(result, 8);
    println!("Arc moved across thread: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::sync::Mutex;

    #[test]
    fn synchronization_changes_shared_access_capabilities() {
        assert_send::<RefCell<u8>>(); // 按值移动与共享访问分别判断。
        assert_sync::<Mutex<RefCell<u8>>>();
        assert_send::<&Mutex<RefCell<u8>>>(); // T: Sync 时 &T: Send。
        assert_send::<Arc<Mutex<RefCell<u8>>>>();
    }
}
