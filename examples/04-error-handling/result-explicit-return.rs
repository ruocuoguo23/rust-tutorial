// 问题：当前函数不负责报告错误时，怎样显式把 Err 交还调用方？
// 输入与状态：解析 8080 时取出数值继续；解析 many 时立即 return Err。
// 结果证据：输出 Ok(8080) 与 invalid_is_err=true；测试覆盖空串和 u16 边界。
// 改一处：在 Ok(port) 前添加打印，观察失败时不会走到那里。
// 边界：返回 Result 不会自动打印或重试；解析合法也不等于符合业务端口策略。
// 运行：cargo run --locked --example 04-result-explicit-return

use std::num::ParseIntError;

fn parse_port(raw: &str) -> Result<u16, ParseIntError> {
    let port = match raw.parse::<u16>() {
        Ok(port) => port,
        Err(error) => return Err(error),
    };
    Ok(port)
}

pub fn main() {
    let valid = parse_port("8080");
    let invalid = parse_port("many");
    assert_eq!(valid, Ok(8080));
    assert!(invalid.is_err());
    println!(
        "Explicit return: valid={valid:?}, invalid_is_err={}",
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
    fn returns_errors_for_empty_invalid_negative_and_overflowing_input() {
        for raw in ["", "many", "-1", "65536"] {
            assert!(parse_port(raw).is_err(), "input {raw:?} should fail");
        }
    }
}
