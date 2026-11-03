// 问题：可变赋值、同名遮蔽与表达式返回值有什么区别？
// 输入与状态：x 从 5 改为 6；spaces 从 &str 遮蔽为 usize；块的最后一个表达式产生 6。
// 结果证据：输出展示两次 x、长度 3 和 Block value: 6。
// 改一处：给 base * 3 加分号，观察块变为 () 后无法使用 Display 输出。
// 边界：遮蔽是建立新绑定；同一可变绑定仍受原类型约束。
// 运行：cargo run --locked --example 01-expressions-shadowing

pub fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len(); // Shadowing can change the binding's type.
    println!("Shadowed value: {spaces}");

    let block_value = {
        let base = 2;
        base * 3 // No semicolon: this block evaluates to 6.
    };
    println!("Block value: {block_value}");
}
