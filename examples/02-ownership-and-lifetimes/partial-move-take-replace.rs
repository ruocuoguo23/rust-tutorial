// 问题：如何移出字段，并给原位置留下合法值？
// 输入与状态：User.name 被移走，age 仍可用；take 留空 String，replace 放入 Grace；Option::take 留 None。
// 结果证据：断言确认空字符串和 None，输出显示移出的旧值与替换后的新值。
// 改一处：取消 let whole = user 的注释，观察部分 Move 后不能使用整个 User。
// 边界：take/replace 保证类型合法，业务上是否允许空值仍需调用方定义。
// 运行：cargo run --locked --example 02-partial-move-take-replace

pub fn main() {
    struct User {
        name: String,
        age: u8,
    }
    let user = User {
        name: String::from("Ada"),
        age: 20,
    };
    let name = user.name;
    println!("Partial move: name={name}, remaining age={}", user.age);
    // let whole = user; // E0382: name was moved; user is no longer a complete value.

    let mut text = name;
    let taken = std::mem::take(&mut text);
    assert!(text.is_empty());
    let previous = std::mem::replace(&mut text, String::from("Grace"));
    assert!(previous.is_empty());
    println!("take: old={taken}; replace: current={text}");

    let mut pending = Some(taken);
    let delivered = pending.take();
    assert!(pending.is_none());
    println!("Option::take: delivered={delivered:?}, remaining={pending:?}");
    // take/replace leave a valid value; its business meaning still needs a contract.
}
