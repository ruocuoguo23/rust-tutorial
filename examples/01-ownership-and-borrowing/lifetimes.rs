// 问题：函数返回引用时，生命周期参数约束了什么？
// 输入与状态：longest 从 rust! 与 go 中返回输入引用；no_dangle 返回拥有所有权的 String。
// 结果证据：输出最长输入，测试覆盖左右输入、等长与空字符串。
// 改一处：把返回 String 改为返回函数内 local 的引用，观察 E0515。
// 边界：生命周期标注约束引用关系，不延长被引用对象的存活时间。
// 运行：cargo run --locked --example 01-lifetimes

fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

fn no_dangle() -> String {
    String::from("owned value")
}

pub fn main() {
    let text = String::from("rust!");
    println!("Longest: {}", longest(&text, "go"));
    println!("Owned return: {}", no_dangle());

    // This cannot compile because the returned reference would outlive local:
    // fn dangling<'a>() -> &'a str {
    //     let local = String::from("temporary");
    //     &local // E0515
    // }
}

#[cfg(test)]
mod tests {
    use super::longest;

    #[test]
    fn returns_a_reference_from_the_inputs() {
        assert_eq!(longest("rust", "go"), "rust");
        assert_eq!(longest("go", "rust"), "rust");
        assert_eq!(longest("ab", "cd"), "ab");
        assert_eq!(longest("", ""), "");
    }
}
