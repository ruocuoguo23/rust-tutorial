// 问题：自定义智能指针为何能借给接受 &str 的函数？
// 输入与状态：Name 持有 String；Deref 返回原字符串的 &str，length 接收这个借用。
// 结果证据：&*name 为 Ada，length(&name) 为 3，空字符串为 0。
// 改一处：删除 Deref 实现，观察 length(&name) 不能再自动转换。
// 边界：Deref 返回借用而非 ownership；str::len 计算字节长度。
// 运行：cargo run --locked --example 04-deref-coercion

use std::ops::Deref;

struct Name(String);

impl Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn length(text: &str) -> usize {
    text.len()
}

pub fn main() {
    println!("P0: Deref coercion");
    let name = Name(String::from("Ada"));
    println!(
        "Deref borrows str: {}, byte length={}",
        &*name,
        length(&name)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deref_borrows_the_original_text() {
        let name = Name(String::from("Ada"));
        assert_eq!(&*name, "Ada");
        assert_eq!(length(&name), 3);
        assert_eq!(length(&Name(String::new())), 0);
    }
}
