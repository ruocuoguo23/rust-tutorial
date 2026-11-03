// 问题：短暂重借用如何结束？T: static 是否要求值永远存在？
// 输入与状态：通过短 &mut 引用追加 ?，再用原引用追加 .；String 满足 static 后仍正常 drop。
// 结果证据：断言得到 rust?. 与 [1, 1]，输出展示重借用与两阶段借用。
// 改一处：在 original.push 后继续使用 short，观察重借用仍有效时的冲突。
// 边界：两阶段借用仅适用于特定隐式借用；T: static 与引用活到进程结束不同。
// 运行：cargo run --locked --example 02-reborrow-static

pub fn main() {
    let mut text = String::from("rust");
    let original = &mut text;
    let short = &mut *original;
    short.push('?');
    original.push('.'); // The shorter reborrow has ended.
    assert_eq!(text, "rust?.");

    let mut sizes = vec![1];
    sizes.push(sizes.len()); // This implicit mutable borrow supports two-phase borrowing.
    assert_eq!(sizes, [1, 1]);

    fn requires_static<T: 'static>(_: &T) {}
    requires_static(&text); // String contains no short-lived references.
    println!("Reborrow: {text}; two-phase borrow: {sizes:?}; owned String: 'static");
    drop(text); // T: 'static does not require this value to live forever.
}
