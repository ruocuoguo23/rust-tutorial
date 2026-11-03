// 问题：怎样按数据形状选择分支，并让循环明确推进到终点？
// 输入与状态：guard 不满足后继续匹配；pop 缩短栈；while/loop 更新计数。
// 结果证据：断言覆盖分支、空切片、弹出顺序、循环结果和借用后的原值。
// 改一处：去掉 classify 中的 Some(_) 分支，观察 guard 无法替代穷尽覆盖。
// 边界：模式不自动推进输入；matches! 只返回 bool，循环仍需终止条件。
// 运行：cargo run --locked --example 01-control-flow-patterns

fn classify(value: Option<u8>, expected: u8) -> &'static str {
    match value {
        Some(n) if n == expected => "expected",
        Some(n @ 1..=9) => {
            assert!(n < 10);
            "small"
        }
        Some(0 | 10) => "edge",
        Some(_) => "other",
        None => "missing",
    }
}

fn first_and_remaining(values: &[i32]) -> Option<(i32, usize)> {
    match values {
        [head, tail @ ..] => Some((*head, tail.len())),
        [] => None,
    }
}

fn pop_all(mut stack: Vec<i32>) -> Vec<i32> {
    let mut popped = Vec::new();
    while let Some(value) = stack.pop() {
        popped.push(value);
    }
    popped
}

fn count_down(mut remaining: u8) -> u8 {
    let mut steps = 0;
    while remaining > 0 {
        remaining -= 1;
        steps += 1;
    }
    steps
}

fn loop_result(limit: u8) -> u16 {
    let mut count = 0;
    loop {
        if count == u16::from(limit) {
            break count * 2;
        }
        count += 1;
    }
}

pub fn main() {
    assert_eq!(classify(Some(5), 5), "expected");
    assert_eq!(classify(Some(5), 7), "small"); // guard 失败，继续下一分支。
    let Some((first, remaining)) = first_and_remaining(&[10, 20, 30]) else {
        return;
    };
    assert_eq!((first, remaining), (10, 2));
    assert_eq!(pop_all(vec![1, 2, 3]), [3, 2, 1]);
    assert_eq!(count_down(3), 3);
    assert_eq!(loop_result(3), 6);

    let name = Some(String::from("Ada"));
    assert!(matches!(&name, Some(text) if text.len() == 3));
    assert_eq!(name.as_deref(), Some("Ada")); // 显式借用，没有交出原值。
    assert!(!matches!(10, 1..=9));
    println!("Control flow: guard fallback, slice pattern, LIFO [3, 2, 1], loop result 6");
}

#[cfg(test)]
mod tests {
    use super::{classify, count_down, first_and_remaining, loop_result, pop_all};

    #[test]
    fn guards_fall_through_to_exhaustive_patterns() {
        for (value, expected, result) in [
            (Some(5), 5, "expected"),
            (Some(5), 7, "small"),
            (Some(1), 7, "small"),
            (Some(9), 7, "small"),
            (Some(0), 7, "edge"),
            (Some(10), 7, "edge"),
            (Some(11), 7, "other"),
            (Some(255), 7, "other"),
            (None, 7, "missing"),
        ] {
            assert_eq!(classify(value, expected), result);
        }
    }

    #[test]
    fn slice_patterns_distinguish_empty_single_and_multiple_elements() {
        assert_eq!(first_and_remaining(&[]), None);
        assert_eq!(first_and_remaining(&[0]), Some((0, 0)));
        assert_eq!(first_and_remaining(&[-1, 2, 3]), Some((-1, 2)));
    }

    #[test]
    fn while_let_consumes_the_stack_until_empty() {
        assert!(pop_all(vec![]).is_empty());
        assert_eq!(pop_all(vec![1]), [1]);
        assert_eq!(pop_all(vec![1, 2, 3]), [3, 2, 1]);
    }

    #[test]
    fn loops_terminate_for_zero_and_boundary_counts() {
        for limit in [0, 1, 3, 255] {
            assert_eq!(count_down(limit), limit);
            assert_eq!(loop_result(limit), u16::from(limit) * 2);
        }
    }

    #[test]
    fn matches_returns_a_boolean_without_consuming_borrowed_input() {
        let value = Some(String::from("Ada"));
        assert!(matches!(&value, Some(text) if text.len() == 3));
        assert!(!matches!(&value, Some(text) if text.is_empty()));
        assert!(!matches!(None::<String>, Some(_)));
        assert_eq!(value.as_deref(), Some("Ada"));
    }
}
