// 问题：通过可变 slice 能修改什么？
// 输入与状态：借用 [10, 20, 30]，把第二项 20 改为 25。
// 结果证据：slice 和原 Vec 都读到 [10, 25, 30]，底层长度没有改变。
// 改一处：尝试对 slice 调用 push，观察该接口不提供容器增长。
// 边界：可变 slice 独占已有元素，不能通过它增加底层元素数。
// 运行：cargo run --locked --example 05-slice-mutation

pub fn main() {
    let mut values = vec![10, 20, 30];
    let slice: &mut [i32] = values.as_mut_slice();
    slice[1] = 25;
    assert_eq!(slice, [10, 25, 30]);
    assert_eq!(slice.len(), 3);
    assert_eq!(values, [10, 25, 30]);
    println!("Changed existing element: {values:?}");
}
