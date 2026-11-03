// 问题：Copy 类型赋值后，修改新位置会影响原位置吗？
// 输入与状态：i32 数值 3 复制到 copied，再只把 copied 加一。
// 结果证据：两个位置都可用，断言和输出分别得到 original=3、copied=4。
// 改一处：把 copied += 1 改为 copied += 2，观察原值仍为 3，而副本断言失败。
// 边界：本例只说明 i32 已实现 Copy；自定义结构体不会因字段全为 Copy 而自动实现。
// 运行：cargo run --locked --example 02-copy-value

pub fn main() {
    let original = 3_i32;
    let mut copied = original;
    copied += 1;
    assert_eq!(original, 3);
    assert_eq!(copied, 4);
    println!("Copy: original={original}, copied={copied}");
}
