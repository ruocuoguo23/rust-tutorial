// 问题：共享借用结束后，能否继续修改同一个 String？
// 输入与状态：rust 被 &str 临时查看；view 最后一次使用后，原 String 追加 !。
// 结果证据：输出为 rust!，单元测试确认共享借用结束后修改成功。
// 改一处：把 println!("{view}") 放到 push 后，观察冲突借用的编译错误。
// 边界：NLL 根据实际使用判断借用范围；共享借用仍有效时不能冲突地修改。
// 运行：cargo run --locked --example 01-borrow-nll

fn edit(text: &mut String) {
    let view: &str = &*text;
    println!("Shared borrow: {view}");
    text.push('!'); // NLL ends view's borrow after its last use.
}

pub fn main() {
    let mut text = String::from("rust");
    edit(&mut text);
    println!("After edit: {text}");
}

#[cfg(test)]
mod tests {
    use super::edit;

    #[test]
    fn shared_borrow_ends_before_mutation() {
        let mut text = String::from("rust");
        edit(&mut text);
        assert_eq!(text, "rust!");
    }
}
