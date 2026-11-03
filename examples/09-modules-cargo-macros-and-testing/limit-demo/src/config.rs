use std::num::ParseIntError;

/// 解析允许带两端空白的 u16 数值。
///
/// ```
/// assert_eq!(limit_demo::parse_limit(" 42 ").unwrap(), 42);
/// ```
pub fn parse_limit(raw: &str) -> Result<u16, ParseIntError> {
    cleaned(raw).parse()
}

fn cleaned(raw: &str) -> &str {
    raw.trim()
}

#[cfg(test)]
mod tests {
    #[test]
    fn trims_and_parses() {
        assert_eq!(super::cleaned(" 42 "), "42");
        assert_eq!(super::parse_limit(" 42 ").unwrap(), 42);
    }

    #[test]
    fn rejects_invalid_text_and_overflow() {
        assert!(super::parse_limit("x").is_err());
        assert!(super::parse_limit("65536").is_err());
    }
}
