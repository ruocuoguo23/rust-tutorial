// 问题：String 键何时移动，Entry 怎样组织插入、更新与默认值构造？
// 输入与状态：查询保留 query；插入和 entry 消费键；rust、go、rust 累计为 2、1。
// 结果证据：断言计数与排序结果；默认值闭包对已有键不执行，对缺失键执行一次。
// 改一处：取消 candidate 的打印注释，观察即使键已存在也不能使用已移动的变量。
// 边界：Entry 不提供线程同步；or_insert_with 延迟构造值，不延迟构造键；遍历顺序不保证。
// 运行：cargo run --locked --example 05-hashmap-entry

use std::collections::HashMap;

pub fn main() {
    println!("P1: HashMap ownership / Entry");
    let mut scores: HashMap<String, i32> = HashMap::new();
    let key = String::from("rust");
    scores.insert(key, 10);
    // println!("{key}"); // E0382：insert 已移动 String。

    let query = String::from("rust");
    assert_eq!(scores.get(query.as_str()), Some(&10));
    println!("Query still usable: {query}");
    let candidate = String::from("rust");
    *scores.entry(candidate).or_insert(0) += 1;
    // println!("{candidate}"); // E0382：命中也会消费候选键。
    assert_eq!(scores.get("rust"), Some(&11));
    let retained = String::from("go");
    scores.insert(retained.clone(), 20);
    assert_eq!(retained, "go");

    let mut counts: HashMap<String, usize> = HashMap::new();
    for word in ["rust", "go", "rust"] {
        let count = counts.entry(word.to_owned()).or_insert(0);
        *count += 1;
    }
    assert_eq!(counts.get("rust"), Some(&2));
    assert_eq!(counts.get("go"), Some(&1));
    println!("Counts: rust={}, go={}", counts["rust"], counts["go"]);
    counts
        .entry("rust".to_owned())
        .and_modify(|n| *n += 1)
        .or_insert(1);
    counts
        .entry("cpp".to_owned())
        .and_modify(|n| *n += 1)
        .or_insert(1);
    assert_eq!(counts.get("rust"), Some(&3));
    assert_eq!(counts.get("cpp"), Some(&1));

    let mut entries: Vec<_> = counts.into_iter().collect();
    entries.sort_unstable_by(|(left, _), (right, _)| left.cmp(right));
    assert_eq!(
        entries,
        [
            ("cpp".to_owned(), 1),
            ("go".to_owned(), 1),
            ("rust".to_owned(), 3)
        ]
    );
    println!("Explicitly sorted entries: {entries:?}");

    let mut builds = 0;
    scores.entry("rust".to_owned()).or_insert_with(|| {
        builds += 1;
        0
    });
    assert_eq!(builds, 0);
    scores.entry("cpp".to_owned()).or_insert_with(|| {
        builds += 1;
        0
    });
    assert_eq!(builds, 1);
    println!("Default value constructed only for the missing key: {builds}");
}
