// 问题：模式相同，额外条件不满足时，match 会走到哪里？
// 输入与状态：同一个 Some(5) 分别与 expected=5、7 比较，另有 None 输入。
// 结果证据：输出 expected、other、missing；测试覆盖 guard 成败及 0、255 边界。
// 改一处：去掉 Some(_) 分支，观察 guard 不能独自完成穷尽覆盖。
// 边界：guard 为 false 会继续尝试后续分支；这里不加入范围或并列模式。
// 运行：cargo run --locked --example 01-match-guard

fn classify(value: Option<u8>, expected: u8) -> &'static str {
    match value {
        Some(number) if number == expected => "expected",
        Some(_) => "other",
        None => "missing",
    }
}

pub fn main() {
    println!("Some(5), expected 5: {}", classify(Some(5), 5));
    println!("Some(5), expected 7: {}", classify(Some(5), 7));
    println!("None, expected 7: {}", classify(None, 7));
}

#[cfg(test)]
mod tests {
    use super::classify;

    #[test]
    fn matching_values_pass_the_guard() {
        assert_eq!(classify(Some(0), 0), "expected");
        assert_eq!(classify(Some(255), 255), "expected");
    }

    #[test]
    fn failed_guards_continue_to_the_next_pattern() {
        assert_eq!(classify(Some(0), 255), "other");
        assert_eq!(classify(Some(255), 0), "other");
    }

    #[test]
    fn missing_values_do_not_match_some() {
        assert_eq!(classify(None, 0), "missing");
        assert_eq!(classify(None, 255), "missing");
    }
}
