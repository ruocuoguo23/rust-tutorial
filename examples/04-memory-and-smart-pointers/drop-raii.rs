// 问题：资源在显式 drop 和作用域退出时按什么顺序释放？
// 输入与状态：先创建 first 再创建 second；显式 drop second 后退出作用域。
// 结果证据：日志先为 [second]，结束后为 [second, first]。
// 改一处：删除 drop(second)，观察离开作用域时仍按逆声明顺序清理。
// 边界：abort、mem::forget 和强引用环可能让 Drop 不执行。
// 运行：cargo run --locked --example 04-drop-raii

use std::cell::RefCell;
use std::rc::Rc;

struct Mark(&'static str, Rc<RefCell<Vec<&'static str>>>);

impl Drop for Mark {
    fn drop(&mut self) {
        self.1.borrow_mut().push(self.0);
    }
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

pub fn main() {
    println!("P0: Drop / RAII");
    println!("Explicit drop, then scope exit: {:?}", drop_order());
    // abort / mem::forget / strong reference cycles can bypass Drop.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_drop_precedes_scope_cleanup() {
        assert_eq!(drop_order(), ["second", "first"]);
    }
}
