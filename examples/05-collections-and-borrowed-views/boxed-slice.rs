// 问题：不再增长的序列，怎样保持拥有而只借出视图？
// 输入与状态：消费 Vec，转为拥有固定长度元素序列的 Box<[i32]>，再借作 slice。
// 结果证据：Box 保有 [10, 20, 30]，借用视图长度为 3。
// 改一处：转移后使用原 values，观察 E0382；把输入改为空 Vec 并调整断言。
// 边界：Box 切片拥有元素，slice 只借用；转换可能调整多余容量，不保证零分配。
// 运行：cargo run --locked --example 05-boxed-slice

pub fn main() {
    let values = vec![10, 20, 30];
    let fixed: Box<[i32]> = values.into_boxed_slice();
    let view: &[i32] = &fixed;
    assert_eq!(view, [10, 20, 30]);
    assert_eq!(view.len(), 3);
    println!("Owned fixed sequence, borrowed view: {view:?}");
}
