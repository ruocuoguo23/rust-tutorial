// 问题：何时复制值，何时转移所有权，Clone 是否一定复制全部资源？
// 输入与状态：i32 赋值后两份绑定可用；String 克隆后原值被消费；Rc 克隆后共享同一分配。
// 结果证据：输出显示 Copy、Move、共享 Clone 的结果和 same allocation=true。
// 改一处：取消最后一次使用 name 的注释，观察 Move 后的 E0382。
// 边界：Copy 不保证复制成本低；Clone 的行为由类型实现决定。
// 运行：cargo run --locked --example 01-move-copy-clone

use std::rc::Rc;

struct BigData {
    data: Vec<i32>,
}

#[derive(Clone)]
struct SharedData {
    name: String,
    big_data: Rc<BigData>,
}

fn inspect(name: &str) -> usize {
    name.len()
}

fn consume(name: String) -> usize {
    name.len()
}

pub fn main() {
    let retries = 3;
    let copied = retries; // i32 implements Copy, so both bindings remain usable.
    println!("Copy: retries={retries}, copied={copied}");

    let name = String::from("Ada");
    let borrowed_len = inspect(&name);
    println!("Borrow: name={name}, len={borrowed_len}");

    // String::clone duplicates this String's buffer. Clone does not universally mean deep copy.
    let backup = name.clone();
    let consumed_len = consume(name);
    println!("Move: backup={backup}, consumed_len={consumed_len}");
    // println!("{name}"); // E0382: name was moved into consume.

    let big_data = Rc::new(BigData {
        data: vec![1; 100000],
    });
    let shared_data1 = SharedData {
        name: "Alice".to_string(),
        big_data: Rc::clone(&big_data),
    };
    let shared_data2 = shared_data1.clone();
    println!(
        "Shared Clone: name={}, len={}, same allocation={}",
        shared_data1.name,
        shared_data1.big_data.data.len(),
        Rc::ptr_eq(&shared_data1.big_data, &shared_data2.big_data)
    );
}
