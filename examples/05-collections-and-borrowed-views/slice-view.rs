// 问题：只读 Vec 的一部分，会改变原容器吗？
// 输入与状态：[10, 20, 30] 借出下标 1 起的 slice，原 Vec 仍有三个元素。
// 结果证据：视图为 [20, 30]，原 Vec 长度仍为 3。
// 改一处：把范围改为 0..0，观察合法空视图与原 Vec 的长度。
// 边界：slice 只描述借用范围，不管理容量或负责释放；越界范围会 panic。
// 运行：cargo run --locked --example 05-slice-view

pub fn main() {
    let values = vec![10, 20, 30];
    let view: &[i32] = &values[1..];
    assert_eq!(view, [20, 30]);
    assert_eq!(values.len(), 3);
    println!("Borrowed slice: {view:?}; owner length: {}", values.len());
}
