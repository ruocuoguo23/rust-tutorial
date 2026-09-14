use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::sync::Arc;

struct Node {
    value: i32,
    parent: Weak<RefCell<Node>>,
}

struct Mark(&'static str, Rc<RefCell<Vec<&'static str>>>);

impl Drop for Mark {
    fn drop(&mut self) {
        self.1.borrow_mut().push(self.0);
    }
}

struct Name(String);

impl Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn length(text: &str) -> usize {
    text.len()
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

    let reading = child.borrow();
    println!(
        "Conflicting mutable borrow rejected: {}",
        child.try_borrow_mut().is_err()
    );
    drop(reading);

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

fn drop_order() -> Vec<&'static str> {
    let log = Rc::new(RefCell::new(Vec::new()));
    {
        let _first = Mark("first", Rc::clone(&log));
        let second = Mark("second", Rc::clone(&log));
        drop(second);
        assert_eq!(*log.borrow(), ["second"]);
    }
    let result = log.borrow().clone();
    result
}

fn main() {
    println!("P0: Stack / Heap / Box");
    let boxed = Box::new([10, 20, 30]);
    let moved_box = boxed; // Move the owner; this does not clone the allocation.
    println!("Box owns an array: {moved_box:?}");

    println!("P0: Rc / Weak / Arc / RefCell");
    shared_ownership();
    let shared = Arc::new(String::from("shared"));
    let other = Arc::clone(&shared);
    assert!(Arc::ptr_eq(&shared, &other));
    println!("Arc strong count: {}", Arc::strong_count(&shared));
    // Arc protects the reference count; chapter 05 adds synchronization for shared mutation.

    println!("P0: Drop / RAII / Deref");
    println!("Explicit drop, then scope exit: {:?}", drop_order());
    // abort / mem::forget / strong reference cycles can bypass Drop.
    let name = Name(String::from("Ada"));
    println!(
        "Deref borrows str: {}, byte length={}",
        &*name,
        length(&name)
    );

    println!("P1: Cell / DST (Dynamically Sized Type)");
    let slot = Cell::new(String::from("old"));
    let old = slot.replace(String::from("new")); // Cell also supports non-Copy values.
    println!("Cell replacement: {old} -> {}", slot.into_inner());
    let fixed: Box<[i32]> = vec![10, 20, 30].into_boxed_slice();
    let view: &[i32] = &fixed;
    println!("Owned Box<[i32]>, borrowed slice length: {}", view.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refcell_rejects_a_conflicting_borrow_then_allows_mutation() {
        let value = RefCell::new(7);
        let reading = value.borrow();
        assert!(value.try_borrow_mut().is_err());
        drop(reading);
        *value.try_borrow_mut().unwrap() = 8;
        assert_eq!(*value.borrow(), 8);
    }

    #[test]
    fn weak_does_not_keep_the_owner_alive() {
        let owner = Rc::new(7);
        let weak = Rc::downgrade(&owner);
        assert_eq!(*weak.upgrade().unwrap(), 7);
        drop(owner);
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn explicit_drop_precedes_scope_cleanup() {
        assert_eq!(drop_order(), ["second", "first"]);
    }

    #[test]
    fn deref_borrows_the_original_text() {
        let name = Name(String::from("Ada"));
        assert_eq!(&*name, "Ada");
        assert_eq!(length(&name), 3);
        assert_eq!(length(&Name(String::new())), 0);
    }
}
