// 问题：iter、iter_mut 与 into_iter 分别把什么交给循环或闭包？
// 输入与状态：先借用读取长度，再可变借用追加 !，最后消费 Vec 并取得每个 String。
// 结果证据：依次得到 [4, 2]、[rust!, go!]、[RUST!, GO!]。
// 改一处：取消注释最后对 words 的访问，观察按值 into_iter 后原 Vec 已被移动。
// 边界：本例的 owned Vec 按值消费；对引用调用 into_iter 可能产出借用，不能只看方法名。
// 运行：cargo run --locked --example 03-iterator-ownership

pub fn main() {
    let mut words = vec![String::from("rust"), String::from("go")];
    let lengths: Vec<usize> = words.iter().map(String::len).collect();
    assert_eq!(lengths, [4, 2]);
    assert_eq!(words[0], "rust");
    println!("iter borrows: {lengths:?}");

    for word in words.iter_mut() {
        word.push('!');
    }
    assert_eq!(words, ["rust!", "go!"]);
    println!("iter_mut changes: {words:?}");

    let upper: Vec<String> = words.into_iter().map(|word| word.to_uppercase()).collect();
    assert_eq!(upper, ["RUST!", "GO!"]);
    println!("into_iter owns: {upper:?}");
    // println!("{words:?}"); // E0382: into_iter consumed the Vec.
}
