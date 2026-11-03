// 问题：只有共享引用，怎样更新一个按值读写的小状态？
// 输入与状态：Cell 中计数为 0，通过共享引用 get 后 set 为 1。
// 结果证据：原 Cell 读到 1，外层绑定不需要 mut。
// 改一处：再执行一次 get + set，观察计数从 1 变 2。
// 边界：get 要求 T: Copy；Cell 不发放内部引用，也不提供跨线程同步。
// 运行：cargo run --locked --example 06-cell-counter

use std::cell::Cell;

pub fn main() {
    let count = Cell::new(0_u32);
    let shared = &count;
    shared.set(shared.get() + 1);
    assert_eq!(count.get(), 1);
    println!("Cell counter through a shared reference: {}", count.get());
}
