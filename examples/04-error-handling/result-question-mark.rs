// 问题：Result 中的 ? 怎样缩短上一例的显式错误返回？
// 输入与状态：仍解析 8080 与 many；Ok 继续执行，Err 提前返回。
// 结果证据：输出与显式返回例一致；测试使用相同成功和失败边界。
// 改一处：在 Ok(port) 前添加打印，观察 Err 路径同样不会执行它。
// 边界：? 只传播错误，所在函数的返回类型必须允许这条 Result 错误路径。
// 运行：cargo run --locked --example 04-result-question-mark

use std::num::ParseIntError;

fn parse_port(raw: &str) -> Result<u16, ParseIntError> {
    let port = raw.parse::<u16>()?;
    Ok(port)
}

pub fn main() {
    let valid = parse_port("8080");
    let invalid = parse_port("many");
    assert_eq!(valid, Ok(8080));
    assert!(invalid.is_err());
    println!(
        "Result ?: valid={valid:?}, invalid_is_err={}",
        invalid.is_err()
    );
}

#[cfg(test)]
mod tests {
    use super::parse_port;

    #[test]
    fn returns_parsed_values_at_the_u16_boundaries() {
        assert_eq!(parse_port("0"), Ok(0));
        assert_eq!(parse_port("8080"), Ok(8080));
        assert_eq!(parse_port("65535"), Ok(65535));
    }

    #[test]
    fn propagates_errors_for_empty_invalid_negative_and_overflowing_input() {
        for raw in ["", "many", "-1", "65536"] {
            assert!(parse_port(raw).is_err(), "input {raw:?} should fail");
        }
    }
}
