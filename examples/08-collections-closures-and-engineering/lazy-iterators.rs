// 问题：迭代器何时执行操作，iter、iter_mut 和 into_iter 怎样影响所有权？
// 输入与状态：先借用读取词长，再通过 iter_mut 加叹号；构造惰性 map 后 collect，最后按值消费 Vec。
// 结果证据：map 构造时 visits=0，collect 后 visits 等于词数；输出分别显示借用读取、修改和拥有元素。
// 改一处：把 lazy.collect() 改为 lazy.take(1).collect()，观察 visits 变为 1，原来的全量访问断言会失败。
// 边界：消费者驱动本例的 map；不同消费者可以短路，into_iter 的元素类型还取决于被迭代的具体类型。
// 运行：cargo run --locked --example 08-lazy-iterators

use std::cell::Cell;

pub fn main() {
    println!("P1 supplement: Iterator / lazy evaluation");
    let mut words = vec![String::from("rust"), String::from("go")];

    let lengths: Vec<usize> = words.iter().map(String::len).collect();
    println!("Lengths from iter: {lengths:?}");

    for word in words.iter_mut() {
        word.push('!');
    }
    println!("After iter_mut: {words:?}");

    let visits = Cell::new(0);
    let lazy = words.iter().map(|word| {
        visits.set(visits.get() + 1);
        word.len()
    });
    assert_eq!(visits.get(), 0);
    let lengths: Vec<_> = lazy.collect();
    assert_eq!(visits.get(), words.len());
    println!(
        "Lazy map: 0 visits before collect, {} after; {lengths:?}",
        visits.get()
    );

    let upper: Vec<String> = words.into_iter().map(|word| word.to_uppercase()).collect();
    println!("Owned items from into_iter: {upper:?}");
    // println!("{words:?}"); // E0382: into_iter moved words.
}
