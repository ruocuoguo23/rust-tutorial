// 问题：Result 怎样把成功值和失败原因交给 match？
// 输入与状态：依次解析 8080、many、70000，并在当前位置处理 Ok 或 Err。
// 结果证据：8080 输出端口；后两个输入分别输出非法数字和越界原因。
// 改一处：把 many 改为 9000，观察它改走 Ok 分支。
// 边界：这里已经在 main 处理错误，没有使用 ? 向调用方传播。
// 运行：cargo run --locked --example 04-result-basics

pub fn main() {
    for raw in ["8080", "many", "70000"] {
        match raw.parse::<u16>() {
            Ok(port) => println!("Result: {raw} -> Ok({port})"),
            Err(error) => println!("Result: {raw} -> Err({error})"),
        }
    }
}
