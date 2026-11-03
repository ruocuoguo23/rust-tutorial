// 问题：map 在构造时执行转换，还是等到消费者拉取元素？
// 输入与状态：拥有 Tom、Alice 的 Vec 交给 into_iter，map 保存名字转长度的闭包。
// 结果证据：先打印准备收集，再处理 Tom、Alice，最后得到 [3, 5]；精确访问计数留在测试。
// 改一处：把 collect 改为 take(1).collect，观察仅处理 Tom，全量结果断言会失败。
// 边界：惰性不代表原 Vec 的所有权还在；本例是 ASCII，String::len 计算的是 UTF-8 字节数。
// 运行：cargo run --locked --example 03-lazy-iterators

pub fn main() {
    let names = vec![String::from("Tom"), String::from("Alice")];
    let lengths_iter = names.into_iter().map(|name| {
        println!("正在处理：{name}");
        name.len()
    });

    println!("准备收集");
    let lengths: Vec<usize> = lengths_iter.collect();
    assert_eq!(lengths, [3, 5]);
    println!("{lengths:?}");
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    #[test]
    fn adapters_wait_for_consumers_and_advance_only_when_pulled() {
        let words = [String::from("rust!"), String::from("go!")];
        let visits = Cell::new(0);
        let mut lazy = words.iter().map(|word| {
            visits.set(visits.get() + 1);
            word.len()
        });
        assert_eq!(visits.get(), 0);
        assert_eq!(lazy.next(), Some(5));
        assert_eq!(visits.get(), 1);
        assert_eq!(lazy.collect::<Vec<_>>(), [3]);
        assert_eq!(visits.get(), 2);
    }

    #[test]
    fn empty_input_does_not_call_the_mapper() {
        let names: Vec<String> = Vec::new();
        let visits = Cell::new(0);
        let lengths: Vec<_> = names
            .into_iter()
            .map(|name| {
                visits.set(visits.get() + 1);
                name.len()
            })
            .collect();
        assert!(lengths.is_empty());
        assert_eq!(visits.get(), 0);
    }
}
