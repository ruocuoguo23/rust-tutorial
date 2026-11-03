// 问题：怎样同时保留未配置、配置有效、配置无效三个结果？
// 输入与状态：None 返回 Ok(None)；8080 返回 Ok(Some(8080))；many 返回 Err。
// 结果证据：输出三种结果；测试覆盖缺失、0、65535、空串、负数和越界。
// 改一处：把 Ok(None) 改为 Ok(Some(8080))，观察缺失如何被默认值替代。
// 边界：Result 决定是否失败，成功后 Option 决定是否有值；这里不选择默认端口。
// 运行：cargo run --locked --example 04-result-option-states

use std::num::ParseIntError;

fn parse_optional_port(raw: Option<&str>) -> Result<Option<u16>, ParseIntError> {
    // let text = raw?; // E0277：外层返回 Result，None 不能自动变成 Err。
    let text = match raw {
        Some(text) => text,
        None => return Ok(None),
    };
    let port = text.parse::<u16>()?;
    Ok(Some(port))
}

pub fn main() {
    println!("Missing config: {:?}", parse_optional_port(None));
    println!("Valid config: {:?}", parse_optional_port(Some("8080")));
    println!(
        "Invalid config is Err: {}",
        parse_optional_port(Some("many")).is_err()
    );
}

#[cfg(test)]
mod tests {
    use super::parse_optional_port;

    #[test]
    fn preserves_normal_absence_without_choosing_a_default() {
        assert_eq!(parse_optional_port(None), Ok(None));
    }

    #[test]
    fn preserves_present_values_including_u16_boundaries() {
        assert_eq!(parse_optional_port(Some("0")), Ok(Some(0)));
        assert_eq!(parse_optional_port(Some("8080")), Ok(Some(8080)));
        assert_eq!(parse_optional_port(Some("65535")), Ok(Some(65535)));
    }

    #[test]
    fn rejects_present_but_invalid_input() {
        for raw in ["", "many", "-1", "65536"] {
            assert!(
                parse_optional_port(Some(raw)).is_err(),
                "input {raw:?} should fail"
            );
        }
    }
}
