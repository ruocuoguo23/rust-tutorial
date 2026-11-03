// 问题：只有共享引用，怎样临时借出内部值并修改？
// 输入与状态：RefCell 保存 7，通过 RefMut 守卫改为 8，离开内部作用域释放守卫。
// 结果证据：后续共享读取得到 8，外层绑定不需要 mut。
// 改一处：把最后的 borrow 移到写守卫仍存活的作用域，观察动态借用冲突。
// 边界：RefCell 在运行时检查冲突，borrow/borrow_mut 失败会 panic；不是跨线程锁。
// 运行：cargo run --locked --example 06-refcell-borrow

use std::cell::RefCell;

pub fn main() {
    let value = RefCell::new(7);
    let shared = &value;
    {
        let mut writing = shared.borrow_mut();
        *writing += 1;
    }
    assert_eq!(*value.borrow(), 8);
    println!("RefCell after the write guard drops: {}", value.borrow());
}
