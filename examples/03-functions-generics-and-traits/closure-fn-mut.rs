// 问题：闭包怎样修改捕获的状态，并在下一次调用接着推进？
// 输入与状态：increment 可变借用 calls，两次调用将计数从 0 推进到 1、2。
// 结果证据：断言并输出两次返回值 1、2；闭包最后使用后可读取计数 2。
// 改一处：移除 increment 绑定的 mut，观察调用需要可变访问而被拒绝（E0596）。
// 边界：FnMut 可重复调用；它需要可变访问闭包，不等于移出捕获值。
// 运行：cargo run --locked --example 03-closure-fn-mut

pub fn main() {
    let mut calls = 0;
    let mut increment = || {
        calls += 1;
        calls
    };
    let first = increment();
    let second = increment();
    assert_eq!((first, second), (1, 2));
    assert_eq!(calls, 2);
    println!("FnMut: {first} -> {second}; calls={calls}");
}
