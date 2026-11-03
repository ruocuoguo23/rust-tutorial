// 问题：学过 Cell/RefCell 单独修改后，记录器、借用冲突与非 Copy 替换怎样应用？
// 输入与状态：&self 记录器追加两条日志；读 guard 存活时拒绝写借用，释放后 7 变 8。
// 结果证据：日志为 hello、rust；try_borrow_mut 冲突后恢复；Cell 返回 old 并保存 new。
// 改一处：把 drop(reading) 移到写借用之后，观察写借用失败。
// 边界：borrow_mut 冲突会 panic；Cell 的 replace 支持非 Copy 值，不会提供引用。
// 运行：cargo run --locked --example 06-interior-mutability

use std::cell::{Cell, RefCell};

// &self 接口需要记录内部状态，修改权限由 RefCell 的守卫管理。
struct Recorder {
    messages: RefCell<Vec<String>>,
}

impl Recorder {
    fn record(&self, text: &str) {
        self.messages.borrow_mut().push(text.to_owned());
    }
}

pub fn p0() {
    println!("P0: RefCell runtime borrowing");
    let recorder = Recorder {
        messages: RefCell::new(Vec::new()),
    };
    let shared = &recorder; // 绑定没有 mut，方法接收的也是共享引用。
    shared.record("hello");
    shared.record("rust");
    assert_eq!(*recorder.messages.borrow(), ["hello", "rust"]);
    println!("Recorded through &self: {:?}", recorder.messages.borrow());
    let value = RefCell::new(7);
    let reading = value.borrow();
    assert!(value.try_borrow_mut().is_err());
    println!("Conflicting mutable borrow rejected: true");
    drop(reading);
    *value.try_borrow_mut().unwrap() = 8;
    assert_eq!(*value.borrow(), 8);
    println!(
        "RefCell value after releasing the read guard: {}",
        value.borrow()
    );
}

pub fn p1() {
    println!("P1: Cell replacement");
    let slot = Cell::new(String::from("old"));
    let old = slot.replace(String::from("new")); // Cell also supports non-Copy values.
    println!("Cell replacement: {old} -> {}", slot.into_inner());
}

pub fn main() {
    p0();
    p1();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recorder_appends_messages_through_a_shared_reference() {
        let recorder = Recorder {
            messages: RefCell::new(Vec::new()),
        };
        let shared = &recorder;
        shared.record("hello");
        shared.record("rust");
        assert_eq!(*recorder.messages.borrow(), ["hello", "rust"]);
    }

    #[test]
    fn refcell_rejects_a_conflicting_borrow_then_allows_mutation() {
        let value = RefCell::new(7);
        let reading = value.borrow();
        assert!(value.try_borrow_mut().is_err());
        drop(reading);
        *value.try_borrow_mut().unwrap() = 8;
        assert_eq!(*value.borrow(), 8);
    }
}
