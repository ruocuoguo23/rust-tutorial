// 问题：已学 Rc、Weak 和 RefCell 后，怎样组织可修改的父子关系？
// 输入与状态：child 的两个 Rc owner 共享 RefCell，值由 2 变 3；Weak 观察 parent。
// 结果证据：child 为 3；删除 parent 的最后一个强 owner 后，upgrade 从 Some 变 None。
// 改一处：保留 parent 的 Rc clone 后再 drop parent，观察 Weak 仍可升级。
// 边界：这是组合应用；Rc 管存活、Weak 管非拥有关系、RefCell 管动态借用，均不提供线程同步。
// 运行：cargo run --locked --example 06-shared-ownership

use std::cell::RefCell;
use std::rc::{Rc, Weak};

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

    let alias = Rc::clone(&child);
    alias.borrow_mut().value += 1;
    assert_eq!(child.borrow().value, 3);
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
    println!("Combined: Rc / Weak / RefCell parent-child graph");
    shared_ownership();
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
