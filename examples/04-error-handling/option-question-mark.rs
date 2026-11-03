// 问题：Option 中的 ? 遇到 None，会从哪里提前返回？
// 输入与状态：None 在取出文本前返回；Some 文本继续提取第一个单词。
// 结果证据：输出 None、Some("hello")、None；测试覆盖空串、空白和前导空白。
// 改一处：把 Some("hello rust") 改为 Some("   ")，观察没有单词也返回 None。
// 边界：本例主动合并缺失文本与没有单词；? 不会把 None 自动变成 Result 的 Err。
// 运行：cargo run --locked --example 04-option-question-mark

fn first_word(raw: Option<&str>) -> Option<&str> {
    let text = raw?;
    text.split_whitespace().next()
}

pub fn main() {
    println!("Missing text: {:?}", first_word(None));
    println!("Present text: {:?}", first_word(Some("hello rust")));
    println!("Blank text: {:?}", first_word(Some("   ")));
}

#[cfg(test)]
mod tests {
    use super::first_word;

    #[test]
    fn missing_empty_and_blank_inputs_have_no_word() {
        assert_eq!(first_word(None), None);
        assert_eq!(first_word(Some("")), None);
        assert_eq!(first_word(Some("   ")), None);
    }

    #[test]
    fn extracts_the_first_word_after_leading_whitespace() {
        assert_eq!(first_word(Some("hello rust")), Some("hello"));
        assert_eq!(first_word(Some("  hello rust")), Some("hello"));
    }
}
