// 问题：如何区分解析失败与业务拒绝，并保留底层错误？
// 输入与状态：8080 成功；0 触发 Zero；many 和 70000 触发 Parse，并保留 ParseIntError。
// 结果证据：输出错误及是否存在 source；测试验证成功边界与两类错误。
// 改一处：将零端口策略改为允许 0，观察业务规则变化而解析规则不变。
// 边界：零端口被拒绝是本例策略；Result 适合可恢复错误，unwrap 可能 panic。
// 运行：cargo run --locked --example 02-typed-errors

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum PortError {
    Parse(ParseIntError),
    Zero,
}

impl fmt::Display for PortError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(error) => write!(formatter, "invalid port: {error}"),
            Self::Zero => write!(formatter, "port zero is excluded by this example's policy"),
        }
    }
}

impl Error for PortError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Parse(error) => Some(error),
            Self::Zero => None,
        }
    }
}

impl From<ParseIntError> for PortError {
    fn from(error: ParseIntError) -> Self {
        Self::Parse(error)
    }
}

fn service_port(raw: &str) -> Result<u16, PortError> {
    let port = raw.parse::<u16>()?; // From converts the parse error, without logging or retrying.
    if port == 0 {
        return Err(PortError::Zero);
    }
    Ok(port)
}

pub fn main() {
    for raw in ["8080", "0", "many", "70000"] {
        match service_port(raw) {
            Ok(port) => println!("Service port: {port}"),
            Err(error) => println!("{raw}: {error}; has source={}", error.source().is_some()),
        }
    }
    // "0".parse::<u16>() is valid u16 parsing; rejection is our service policy, not a Rust rule.
    // "many".parse::<u16>().unwrap(); // Would panic; recoverable input errors use Result.
}

#[cfg(test)]
mod tests {
    use super::{service_port, PortError};
    use std::error::Error;

    #[test]
    fn distinguishes_parse_failures_from_policy_failures() {
        assert_eq!(service_port("1").unwrap(), 1);
        assert_eq!(service_port("65535").unwrap(), 65535);
        let zero = service_port("0").unwrap_err();
        assert!(matches!(zero, PortError::Zero));
        assert!(zero.source().is_none());
        for raw in ["many", "65536", ""] {
            let error = service_port(raw).unwrap_err();
            assert!(matches!(error, PortError::Parse(_)));
            assert!(error.source().is_some());
        }
    }
}
