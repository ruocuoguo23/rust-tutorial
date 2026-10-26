// 问题：Rc / Arc clone 后共享谁，Weak 是否延长目标存活？
// 输入与状态：child 用 Weak 指向 parent；删除 parent owner 后再次 upgrade。
// 结果证据：Weak upgrade 从 Some 变为 None；两个 Arc 指向同一分配。
// 改一处：保留 parent 的 Rc clone 后再 drop parent，观察 Weak 仍可升级。
// 边界：Arc 只同步引用计数，共享可变数据仍需锁；RefCell 检查借用冲突。
// 运行：cargo run --locked --example 04-shared-ownership

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::Arc;

struct Node {
    value: i32,
    parent: Weak<RefCell<Node>>,
}

fn shared_ownership() {
    let parent = Rc::new(RefCell::new(Node {
        value: 1,
        parent: Weak::new(),
    }));
    let child = Rc::new(RefCell::new(Node {
        value: 2,
        parent: Weak::new(),
    }));
    child.borrow_mut().parent = Rc::downgrade(&parent);

    child.borrow_mut().value += 1;
    println!("Child value: {}", child.borrow().value);
    println!(
        "Parent alive through Weak: {}",
        child.borrow().parent.upgrade().is_some()
    );

    drop(parent);
    println!(
        "Parent alive after drop: {}",
        child.borrow().parent.upgrade().is_some()
    );
}

pub fn main() {
    println!("P0: Rc / Weak / Arc / RefCell");
    shared_ownership();
    let shared = Arc::new(String::from("shared"));
    let other = Arc::clone(&shared);
    assert!(Arc::ptr_eq(&shared, &other));
    println!("Arc strong count: {}", Arc::strong_count(&shared));
    // Arc protects the reference count; chapter 05 adds synchronization for shared mutation.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weak_does_not_keep_the_owner_alive() {
        let owner = Rc::new(7);
        let weak = Rc::downgrade(&owner);
        assert_eq!(*weak.upgrade().unwrap(), 7);
        drop(owner);
        assert!(weak.upgrade().is_none());
    }
}
