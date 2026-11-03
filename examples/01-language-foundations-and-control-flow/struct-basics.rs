// 问题：怎样把同时存在的字段放进一个 Struct，再按名称读取？
// 输入与状态：坐标 3 和 4 构造为同一个 Point，不涉及分支或状态转换。
// 结果证据：断言分别读取 x=3、y=4，输出 Point fields: x=3, y=4。
// 改一处：把构造时的 x 改为 -3，同步观察字段读取与断言。
// 边界：Struct 组合字段，不自动验证坐标是否符合业务范围。
// 运行：cargo run --locked --example 01-struct-basics

struct Point {
    x: i32,
    y: i32,
}

pub fn main() {
    let point = Point { x: 3, y: 4 };
    assert_eq!(point.x, 3);
    assert_eq!(point.y, 4);
    println!("Point fields: x={}, y={}", point.x, point.y);
}
