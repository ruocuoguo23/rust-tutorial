// 问题：怎样给已有序列增加一个元素？
// 输入与状态：Vec 从 [10, 20, 30] 开始，push(40) 后长度从 3 变成 4。
// 结果证据：输出和断言确认新元素成为已有内容。
// 改一处：改为空 Vec，再依次 push 两个值，观察长度如何推进。
// 边界：push 需要可变访问 Vec，超出容量可能重新分配；容量不等于已有元素数。
// 运行：cargo run --locked --example 05-vec-growth

pub fn main() {
    let mut values = vec![10, 20, 30];
    assert_eq!(values.len(), 3);
    values.push(40);
    assert_eq!(values, [10, 20, 30, 40]);
    assert_eq!(values.len(), 4);
    println!("Vec after push: {values:?}; length: {}", values.len());
}
