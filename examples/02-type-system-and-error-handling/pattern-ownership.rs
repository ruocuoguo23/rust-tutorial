// 问题：模式是借用值、忽略值，还是移动值？
// 输入与状态：先借用 Some 中的 String；let _ = value 后仍可用；命名绑定 _owned 移走 value。
// 结果证据：assert!(value.is_some()) 证明通配符没有移动该位置的值。
// 改一处：取消最后打印 value 的注释，观察移动到命名绑定后的 E0382。
// 边界：这里讨论已有位置表达式；下划线开头的名字仍然是实际绑定。
// 运行：cargo run --locked --example 02-pattern-ownership

pub fn main() {
    let value = Some(String::from("kept by borrowing"));
    if let Some(name) = &value {
        println!("Borrowed pattern: {name}");
    }
    let _ = value; // Wildcard does not take ownership of this place.
    assert!(value.is_some());
    let _owned = value; // A named binding moves this non-Copy value.
                        // println!("{value:?}"); // E0382: value was moved into _owned.
}
