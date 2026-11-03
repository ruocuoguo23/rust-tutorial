// 问题：Rc clone 后，多出来的 owner 拥有另一份值吗？
// 输入与状态：owner 与 alias 共享同一个 String，释放 owner 后仍有 alias。
// 结果证据：ptr_eq 为 true，强计数 2 变 1，alias 仍读到 rust。
// 改一处：再 clone 一次，观察强计数；比较 String::clone 与 Rc::clone 的目标。
// 边界：Rc 不提供内部值的任意修改，也不是跨线程共享计数工具。
// 运行：cargo run --locked --example 06-rc-sharing

use std::rc::Rc;

pub fn main() {
    let owner = Rc::new(String::from("rust"));
    let alias = Rc::clone(&owner);
    assert!(Rc::ptr_eq(&owner, &alias));
    assert_eq!(Rc::strong_count(&owner), 2);
    drop(owner);
    assert_eq!(alias.as_str(), "rust");
    assert_eq!(Rc::strong_count(&alias), 1);
    println!(
        "Shared target: {}; remaining owner count: 1",
        alias.as_str()
    );
}
