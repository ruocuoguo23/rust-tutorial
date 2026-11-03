// 问题：Arc 的原子计数保护了谁？
// 输入与状态：clone Arc 共享同一个 String，释放原 owner 后由 alias 保持存活。
// 结果证据：ptr_eq 为 true，强计数 2 变 1，alias 仍能读到 rust。
// 改一处：保留两个 owner，观察 strong_count 为 2。
// 边界：Arc 只同步存活计数，不自动同步内部修改；跨线程仍要求 T 满足 Send/Sync。
// 运行：cargo run --locked --example 06-arc-sharing

use std::sync::Arc;

pub fn main() {
    let owner = Arc::new(String::from("rust"));
    let alias = Arc::clone(&owner);
    assert!(Arc::ptr_eq(&owner, &alias));
    assert_eq!(Arc::strong_count(&owner), 2);
    drop(owner);
    assert_eq!(alias.as_str(), "rust");
    assert_eq!(Arc::strong_count(&alias), 1);
    println!(
        "Arc shares one target: {}; remaining owner count: 1",
        alias.as_str()
    );
}
