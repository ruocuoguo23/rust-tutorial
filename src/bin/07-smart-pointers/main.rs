use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    value: i32,
    parent: Weak<RefCell<Node>>,
}

fn main() {
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
