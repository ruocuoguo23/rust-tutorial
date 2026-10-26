// 问题：derive 提供的行为与手写相等规则有什么区别？
// 输入与状态：Point 派生 Debug/Clone；自定义 PartialEq 只比较 x，因此 y 不同也能相等。
// 结果证据：输出克隆对象与 equality=true；测试确认 x 相同相等、x 不同不等。
// 改一处：把 PartialEq 改为同时比较 y，观察现有相等断言失败。
// 边界：自定义相等是该类型的约定；derive 不会自动猜测业务相等规则。
// 运行：cargo run --locked --example 03-derive-equality

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

pub fn main() {
    let point1 = Point { x: 1, y: 2 };
    let point2 = point1.clone();
    let point3 = Point { x: 1, y: 3 };
    println!("Derived Debug and Clone: {point1:?}, {point2:?}");
    println!("Manual PartialEq compares x only: {}", point1 == point3);
    println!(
        "Different y values do not affect this equality: {}, {}",
        point1.y, point3.y
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manual_equality_compares_only_the_declared_field() {
        assert_eq!(Point { x: 1, y: 2 }, Point { x: 1, y: 3 });
        assert_ne!(Point { x: 1, y: 2 }, Point { x: 2, y: 2 });
    }
}
