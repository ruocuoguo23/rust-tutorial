// 问题：一次加锁怎样取得访问权，什么时候释放？
// 输入与状态：Mutex 中从 0 加到 1；guard 离开内部作用域。
// 结果证据：退出作用域后能重新加锁，读到的值仍为 1。
// 改一处：把读取语句移到作用域内，通过已有 guard 读取同一个值。
// 边界：本例没有线程和中毒恢复；guard 的存活范围决定本次持锁范围。
// 运行：cargo run --locked --example 07-mutex-basics

use std::sync::Mutex;

pub fn main() {
    let count = Mutex::new(0);
    {
        let mut guard = count.lock().unwrap();
        *guard += 1;
        println!("holding guard: {}", *guard);
    } // guard 被释放，锁可以再次获取。
    let guard = count.lock().unwrap();
    assert_eq!(*guard, 1);
    println!("locked again: {}", *guard);
}
