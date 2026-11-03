// 问题：Option 的有值与缺失分支分别由谁处理？
// 输入与状态：Some(9000) 采用原值；None 由本例选择默认端口 8080。
// 结果证据：输出 9000 与 8080；测试还确认 Some(0) 不会被误认为缺失。
// 改一处：把 None 分支的 8080 改为 3000，观察默认策略变化。
// 边界：None 只表示缺失，Option 不会自动提供默认值或把缺失当作错误。
// 运行：cargo run --locked --example 04-option-basics

fn port_or_default(configured: Option<u16>) -> u16 {
    match configured {
        Some(port) => port,
        None => 8080,
    }
}

pub fn main() {
    println!("Present port: {}", port_or_default(Some(9000)));
    println!("Missing port uses default: {}", port_or_default(None));
}

#[cfg(test)]
mod tests {
    use super::port_or_default;

    #[test]
    fn keeps_present_values_including_zero() {
        assert_eq!(port_or_default(Some(0)), 0);
        assert_eq!(port_or_default(Some(9000)), 9000);
        assert_eq!(port_or_default(Some(u16::MAX)), u16::MAX);
    }

    #[test]
    fn chooses_the_example_default_only_for_none() {
        assert_eq!(port_or_default(None), 8080);
    }
}
