// 问题：Weak 会让目标在最后一个强 owner 释放后继续存活吗？
// 输入与状态：建立 Weak，先成功 upgrade，再释放唯一强 owner。
// 结果证据：strong count 仍为 1；释放后 upgrade 从 Some 变为 None。
// 改一处：把 upgrade 得到的 Rc 保存到变量，观察它也能延长目标存活。
// 边界：Weak 不增加强计数；upgrade 成功得到的 Rc 会成为新的强 owner。
// 运行：cargo run --locked --example 06-weak-lifecycle

use std::rc::Rc;

pub fn main() {
    let owner = Rc::new(7);
    let observer = Rc::downgrade(&owner);
    assert_eq!(Rc::strong_count(&owner), 1);
    assert!(observer.upgrade().is_some());
    println!("Weak upgrade before last owner drops: Some");
    drop(owner);
    assert!(observer.upgrade().is_none());
    println!("Weak upgrade after last owner drops: None");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[test]
    fn an_upgraded_owner_delays_destruction() {
        let owner = Rc::new(7);
        let observer = Rc::downgrade(&owner);
        let upgraded = observer.upgrade().unwrap();
        drop(owner);
        assert_eq!(*upgraded, 7);
        assert!(observer.upgrade().is_some());
        drop(upgraded);
        assert!(observer.upgrade().is_none());
    }

    #[test]
    fn last_strong_owner_drops_value_while_weak_remains() {
        struct Probe(Rc<Cell<usize>>);
        impl Drop for Probe {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }
        let drops = Rc::new(Cell::new(0));
        let owner = Rc::new(Probe(Rc::clone(&drops)));
        let alias = Rc::clone(&owner);
        let observer = Rc::downgrade(&owner);
        drop(owner);
        assert_eq!(drops.get(), 0);
        drop(alias);
        assert_eq!(drops.get(), 1);
        assert!(observer.upgrade().is_none());
        drop(observer);
        assert_eq!(drops.get(), 1);
    }
}
