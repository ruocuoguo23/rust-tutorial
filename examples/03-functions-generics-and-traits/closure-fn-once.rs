// 问题：闭包把捕获的 String 移出后，为什么不能再取一次？
// 输入与状态：text 先 move 到 take，调用 take 时再把 String 交给 owned。
// 结果证据：owned 得到 rust；take 在第一次调用后已被消费。
// 改一处：取消注释第二次 take()，观察 E0382，而不执行无效的重复消费。
// 边界：本闭包只能按 FnOnce 调用；所有 FnOnce 实现者不一定都只能调用一次。
// 运行：cargo run --locked --example 03-closure-fn-once

pub fn main() {
    let text = String::from("rust");
    let take = move || text;
    let owned = take();
    assert_eq!(owned, "rust");
    println!("FnOnce returned ownership: {owned}");
    // take(); // E0382: the first call consumed the closure.
}
