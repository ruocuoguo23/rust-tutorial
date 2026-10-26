// 问题：借用 Vec 元素后，何时可以改变容器？
// 输入与状态：先读取 [10,20,30] 的第一个元素，结束借用后 push(40)。
// 结果证据：输出和断言确认 Vec 变为 [10,20,30,40]。
// 改一处：取消注释错误对照，让 first 在 push 后仍被使用，观察共享借用和可变借用冲突。
// 边界：push 可能重新分配；本例说明借用约束，不保证某次 push 实际发生地址变化。
// 运行：cargo run --locked --example 08-vec-slice

pub fn main() {
    println!("P1 supplement: Vec / slice");
    let mut values = vec![10, 20, 30];
    {
        let first = &values[0];
        println!("Borrowed first value: {first}");
    }
    values.push(40); // The element borrow ended before Vec was mutated.
    println!("Vec after push: {values:?}");

    // The following order cannot compile because push needs &mut Vec while first is live:
    // let first = &values[0];
    // values.push(50); // E0502
    // println!("{first}");

    assert_eq!(values, [10, 20, 30, 40]);
}
