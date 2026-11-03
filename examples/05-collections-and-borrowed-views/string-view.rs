// 问题：读取 String 时，需要转交它的所有权吗？
// 输入与状态：String 拥有 rust，as_str 建立只读视图，原容器保持可用。
// 结果证据：视图和原 String 都读到 rust，没有复制文本。
// 改一处：把 as_str 改为 to_owned，观察 &str 声明不能接收 String 的类型错误。
// 边界：视图不能超过底层文本的有效期；建立视图不会复制缓冲区。
// 运行：cargo run --locked --example 05-string-view

pub fn main() {
    let text = String::from("rust");
    let view: &str = text.as_str();
    assert_eq!(view, "rust");
    assert_eq!(text, "rust");
    println!("Borrowed text: {view}; owner still holds: {text}");
}
