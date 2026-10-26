// 问题：同一行为如何通过泛型、dyn Trait 和 impl Trait 使用？
// 输入与状态：u32/bool 实现 Encoder；泛型与 Trait object 返回字符串；impl Iterator 隐藏固定 Range 类型。
// 结果证据：输出静态和动态路径，测试确认编码结果一致且迭代得到 [0, 1, 2]。
// 改一处：给 items 增加返回另一迭代器类型的分支，观察 impl Trait 的固定具体类型限制。
// 边界：借用 &dyn Trait 本身不要求分配；Box 在此用于容纳不同类型。
// 运行：cargo run --locked --example 03-trait-dispatch

trait Encoder {
    fn encode(&self) -> String;
}

impl Encoder for u32 {
    fn encode(&self) -> String {
        self.to_string()
    }
}

impl Encoder for bool {
    fn encode(&self) -> String {
        self.to_string()
    }
}

fn encode_static<T: Encoder>(value: &T) -> String {
    value.encode() // The concrete type is known at this call site.
}

fn encode_dynamic(value: &dyn Encoder) -> String {
    value.encode() // The implementation is selected through a trait object.
}

fn items() -> impl Iterator<Item = u8> {
    0..3 // Return-position impl Trait hides this single concrete Range type.
}

pub fn main() {
    let number = 42_u32;
    println!("Static dispatch: {}", encode_static(&number));
    println!("Borrowed dynamic dispatch: {}", encode_dynamic(&number));
    // &dyn Encoder itself does not require a new heap allocation.

    let values: Vec<Box<dyn Encoder>> = vec![Box::new(7_u32), Box::new(true)];
    for value in values {
        println!("Dynamic dispatch: {}", encode_dynamic(value.as_ref()));
    }

    println!("impl Iterator: {:?}", items().collect::<Vec<_>>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dispatch_paths_preserve_the_encoder_contract() {
        assert_eq!(encode_static(&42_u32), "42");
        assert_eq!(encode_dynamic(&42_u32), "42");
        assert_eq!(encode_static(&false), "false");
        assert_eq!(encode_dynamic(&true), "true");
        assert_eq!(items().collect::<Vec<_>>(), [0, 1, 2]);
    }
}
