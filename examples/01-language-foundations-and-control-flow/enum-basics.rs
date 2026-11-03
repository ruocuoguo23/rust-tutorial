// 问题：怎样构造同一个 Enum 类型的不同变体？
// 输入与状态：分别构造 Pending 和 Done；每个 Status 值只处于一种状态。
// 结果证据：两行输出分别显示 Pending 与 Done。
// 改一处：把 waiting 的构造改为 Status::Done，观察打印的变体。
// 边界：这里仅构造两个值，不证明从 Pending 到 Done 的业务转换是否合法。
// 运行：cargo run --locked --example 01-enum-basics

#[derive(Debug)] // 只用于把变体打印出来观察。
enum Status {
    Pending,
    Done,
}

pub fn main() {
    let waiting = Status::Pending;
    let finished = Status::Done;
    println!("Waiting value: {waiting:?}");
    println!("Finished value: {finished:?}");
}
