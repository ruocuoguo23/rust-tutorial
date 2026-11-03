// 问题：只读取外部值的闭包能否重复调用，原值还能否使用？
// 输入与状态：length 共享借用 String，每次读取相同的字节长度。
// 结果证据：两次调用都得到 4，外部 text 仍是 rust。
// 改一处：增加 move 捕获，观察闭包仍可重复调用，但末尾访问外部 text 会触发 E0382。
// 边界：Fn 允许共享访问调用；move 决定捕获方式，不自动决定调用次数。
// 运行：cargo run --locked --example 03-closure-fn

pub fn main() {
    let text = String::from("rust");
    let length = || text.len();
    let first = length();
    let second = length();
    assert_eq!((first, second), (4, 4));
    assert_eq!(text, "rust");
    println!("Fn: {first}, {second}; text={text}");
}
