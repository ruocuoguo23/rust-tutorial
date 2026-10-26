// 问题：只读时如何借用输入，需要修改时如何才创建拥有的数据？
// 输入与状态：normalize(rust) 保留借用，normalize(Rust) 返回拥有的小写字符串；to_mut 将借用变成拥有。
// 结果证据：断言 Borrowed/Owned 分支和第一次写入后的 rust!；测试覆盖空串、ASCII 大写和非 ASCII。
// 改一处：把 Owned 断言中的 "Rust" 改成 "rust"，观察断言失败：这个输入会走 Borrowed 分支。
// 边界：本例仅做 ASCII 大小写转换；Cow 不保证所有修改都零拷贝，也不是 Unicode 标准化。
// 运行：cargo run --locked --example 08-cow

use std::borrow::Cow;

fn normalize(text: &str) -> Cow<'_, str> {
    if text.bytes().any(|byte| byte.is_ascii_uppercase()) {
        Cow::Owned(text.to_ascii_lowercase())
    } else {
        Cow::Borrowed(text)
    }
}

pub fn main() {
    println!("P1 supplement: Cow (Clone-on-Write)");
    assert!(matches!(normalize("rust"), Cow::Borrowed(_)));
    assert!(matches!(normalize("Rust"), Cow::Owned(_)));
    println!(
        "normalize: rust -> Borrowed; Rust -> Owned({})",
        normalize("Rust")
    );
    let mut text = Cow::Borrowed("rust");
    text.to_mut().push('!');
    assert!(matches!(text, Cow::Owned(_)));
    println!("First write changes Borrowed to Owned: {text}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalization_borrows_unchanged_input_and_owns_ascii_changes() {
        for text in ["rust", "", "世界", "É"] {
            assert!(matches!(normalize(text), Cow::Borrowed(_)));
            assert_eq!(normalize(text), text); // This is ASCII case folding, not Unicode normalization.
        }
        for (input, expected) in [("Rust", "rust"), ("RUST", "rust"), ("世界A", "世界a")] {
            let normalized = normalize(input);
            assert!(matches!(normalized, Cow::Owned(_)));
            assert_eq!(normalized, expected);
        }
    }
}
