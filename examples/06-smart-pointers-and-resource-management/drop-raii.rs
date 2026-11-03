// 问题：显式 drop 和作用域结束，分别在何时触发析构？
// 输入与状态：创建 first、second，显式释放 second，再离开 first 所在作用域。
// 结果证据：输出为 drop second、leaving scope、drop first、scope ended。
// 改一处：删除 drop(second)，观察 second 改为在 leaving scope 之后析构。
// 边界：abort、mem::forget 和强引用环可绕过 Drop；精确日志断言只放在测试中。
// 运行：cargo run --locked --example 06-drop-raii

struct Mark(&'static str);

impl Drop for Mark {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

pub fn main() {
    {
        let _first = Mark("first");
        let second = Mark("second");
        drop(second);
        println!("leaving scope");
    }
    println!("scope ended");
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    // 共享日志只用于自动验证，避免把观察工具混入第一个 Drop 示例。
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

    #[test]
    fn explicit_drop_precedes_scope_cleanup() {
        assert_eq!(drop_order(), ["second", "first"]);
    }
}
