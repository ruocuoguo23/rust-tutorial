// 问题：宏展开重复使用表达式时，怎样避免重复执行副作用？
// 输入与状态：twice! 的参数先将 calls 加一并返回 3，宏把结果绑定一次，再计算 value+value。
// 结果证据：输出和测试断言 doubled=6、calls=1。
// 改一处：把宏体改为直接写 ($expression) + ($expression)，观察 calls=1 的断言失败：参数被求值两次，calls 变为 2。
// 边界：宏执行的是语法展开而非函数调用；本例只演示整数表达式的求值次数。
// 运行：cargo run --locked --example 09-macro-evaluation

macro_rules! twice {
    ($expression:expr) => {{
        let value = $expression; // Evaluate side effects once, then use the integer twice.
        value + value
    }};
}

pub fn main() {
    println!("P1: Macro / expression evaluation");
    let mut calls = 0;
    let doubled = twice!({
        calls += 1;
        3
    });
    assert_eq!((doubled, calls), (6, 1));
    println!("twice!: value={doubled}, expression evaluations={calls}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn macro_evaluates_its_argument_once() {
        let mut calls = 0;
        assert_eq!(
            twice!({
                calls += 1;
                3
            }),
            6
        );
        assert_eq!(calls, 1);
    }

    #[test]
    fn direct_expansion_repeats_the_argument() {
        macro_rules! repeated {
            ($expression:expr) => {
                ($expression) + ($expression)
            };
        }
        let mut calls = 0;
        let doubled = repeated!({
            calls += 1;
            3
        });
        assert_eq!((doubled, calls), (6, 2));
    }
}
