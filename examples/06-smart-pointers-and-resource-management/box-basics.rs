// 问题：移动 Box 会复制它拥有的目标吗？
// 输入与状态：Box 拥有整数 7，把 owner 从 boxed 转交给 moved。
// 结果证据：新 owner 仍能读取 7，旧 owner 已不能使用。
// 改一处：在移动后读取 boxed，观察 E0382。
// 边界：Box 管理间接拥有的目标；不提供 Pin 的地址稳定契约，也不承诺可观察的分配次数。
// 运行：cargo run --locked --example 06-box-basics

pub fn main() {
    let boxed = Box::new(7_i32);
    let moved = boxed;
    assert_eq!(*moved, 7);
    println!("New Box owner reads: {}", *moved);
}
