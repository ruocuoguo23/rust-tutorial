// 问题：为什么 newtype 能实现外部 Trait，From 与 TryFrom 又各负责什么？
// 输入与状态：&str 经 From/Into 得到本地 Label；256_u16 转 u8 返回 Err。
// 结果证据：输出 Label，断言失败转换；测试覆盖普通与空字符串。
// 改一处：把 u8::try_from 的输入改为 255，观察它转换成功。
// 边界：类型别名不会创建本地类型身份；From 不表达失败，TryFrom 表达失败。
// 运行：cargo run --locked --example 03-newtype-conversions

use std::fmt;

struct Label(String);

impl From<&str> for Label {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl fmt::Display for Label {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

pub fn main() {
    let label: Label = "rust".into(); // From provides the corresponding Into implementation.
    println!("Local newtype Display: {label}");
    assert!(u8::try_from(256_u16).is_err());
    // impl Display for Vec<String> { ... } // E0117: both trait and Self are foreign.
    // A type alias would not create the local identity that Label provides.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newtype_conversion_keeps_its_declared_meaning() {
        let label: Label = "rust".into();
        assert_eq!(label.to_string(), "rust");
        assert_eq!(Label::from("").to_string(), "");
    }
}
