// 问题：String 赋给另一个变量后，谁还能使用这个值？
// 输入与状态：original 持有 Ada；赋值把所有权转交给 moved。
// 结果证据：断言和输出通过 moved 读取 Ada；原位置的使用保留为拒编对照。
// 改一处：取消最后一行注释，观察使用 original 的 E0382。
// 边界：move 不等于复制整段文本，也不要求值一定存放在 heap。
// 运行：cargo run --locked --example 02-move-value

pub fn main() {
    let original = String::from("Ada");
    let moved = original;
    assert_eq!(moved, "Ada");
    println!("Moved owner: {moved}");
    // println!("{original}"); // E0382：original 的值已 move。
}
