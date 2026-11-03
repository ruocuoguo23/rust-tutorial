// 问题：缺少输入、有效输入与无效输入如何用 Option、Result 和 ? 表达？
// 输入与状态：None 使用默认端口 8080；9000 解析成功；many 返回 Err；Option 中的 ? 保留缺失。
// 结果证据：输出四类结果；测试覆盖默认值、边界端口、无效数字、空白和缺失单词。
// 改一处：把 Some("9000") 改为 Some("many")，观察 ? 从 main 提前返回 Err。
// 边界：? 只传播 None/Err，不自动记录日志、重试或变为 panic。
// 运行：cargo run --locked --example 04-option-result-propagation

use std::num::ParseIntError;

fn parse_port(raw: &str) -> Result<u16, ParseIntError> {
    let port = raw.parse::<u16>()?;
    Ok(port)
}

fn configured_port(raw: Option<&str>) -> Result<u16, ParseIntError> {
    match raw {
        Some(value) => parse_port(value),
        None => Ok(8080),
    }
}

fn first_word(raw: Option<&str>) -> Option<&str> {
    raw?.split_whitespace().next()
}

pub fn main() -> Result<(), ParseIntError> {
    println!("Configured port: {}", configured_port(Some("9000"))?);
    println!("Default port: {}", configured_port(None)?);
    println!("Invalid input is Err: {}", parse_port("many").is_err());
    println!(
        "Option ?: {:?}, {:?}",
        first_word(Some("hello rust")),
        first_word(None)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{configured_port, first_word, parse_port};

    #[test]
    fn parses_valid_and_rejects_invalid_ports() {
        assert_eq!(parse_port("8080").unwrap(), 8080);
        assert!(parse_port("many").is_err());
        assert_eq!(parse_port("0").unwrap(), 0);
        assert_eq!(parse_port("65535").unwrap(), 65535);
        assert!(parse_port("65536").is_err());
        assert!(parse_port("-1").is_err());
        assert!(parse_port("").is_err());
    }

    #[test]
    fn uses_a_default_when_the_option_is_none() {
        assert_eq!(configured_port(None).unwrap(), 8080);
        assert_eq!(configured_port(Some("9000")).unwrap(), 9000);
        assert!(configured_port(Some("many")).is_err());
    }

    #[test]
    fn option_question_mark_preserves_missing_and_empty_inputs() {
        assert_eq!(first_word(None), None);
        assert_eq!(first_word(Some("  ")), None);
        assert_eq!(first_word(Some(" hello rust")), Some("hello"));
    }
}
