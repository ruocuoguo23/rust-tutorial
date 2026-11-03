// 问题：怎样用 match 为每种输入选择一个结果？
// 输入与状态：剩余次数为 0、1 或更大值，分别选择 stop、last retry、retry。
// 结果证据：输出三条分支结果；测试覆盖 0、1、2 与 u8 最大值。
// 改一处：删除 _ 分支，观察未覆盖其他 u8 值的 E0004。
// 边界：match 要穷尽输入类型；本例只做选择，不会自动递减剩余次数。
// 运行：cargo run --locked --example 01-match-basics

fn action(retries: u8) -> &'static str {
    match retries {
        0 => "stop",
        1 => "last retry",
        _ => "retry",
    }
}

pub fn main() {
    for retries in [0, 1, 2] {
        println!("Retries {retries}: {}", action(retries));
    }
}

#[cfg(test)]
mod tests {
    use super::action;

    #[test]
    fn distinguishes_zero_one_and_remaining_counts() {
        assert_eq!(action(0), "stop");
        assert_eq!(action(1), "last retry");
        assert_eq!(action(2), "retry");
        assert_eq!(action(u8::MAX), "retry");
    }
}
