// 问题：怎样让同一函数接收不同类型，并返回传入的值？
// 输入与状态：identity 分别接收 u32 与 String；String 的所有权传入后再返回。
// 结果证据：断言并输出 7 与 rust；原 String 绑定已被移动。
// 改一处：取消注释对 name 的访问，观察 E0382，而不是认为泛型绕过了所有权。
// 边界：T 表示一次调用中的同一种类型；没有约束时不能随意调用 T 的方法。
// 运行：cargo run --locked --example 03-generic-basics

fn identity<T>(value: T) -> T {
    value
}

pub fn main() {
    let number = identity(7_u32);
    let name = String::from("rust");
    let returned = identity(name);
    assert_eq!(number, 7);
    assert_eq!(returned, "rust");
    println!("Generic: number={number}, returned={returned}");
    // println!("{name}"); // E0382: identity consumed the original binding.
}
