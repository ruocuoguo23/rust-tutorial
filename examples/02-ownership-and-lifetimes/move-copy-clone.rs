// 问题：何时复制值，何时转移所有权，Clone 是否一定复制全部资源？
// 输入与状态：i32 可 Copy；整数字段结构体须显式派生 Copy；String 字段只支持 Clone。
// 结果证据：输出显示 Copy、Move、共享 Clone 的结果和 same allocation=true。
// 改一处：取消使用 plain1 的注释，观察全整数字段也会整体 Move；给 NamedPoint 加 Copy 观察 E0204。
// 边界：字段为 Copy 只是实现资格，不自动赋予结构体 Copy；Clone 不会被普通赋值隐式调用。
// 运行：cargo run --locked --example 02-move-copy-clone

use std::rc::Rc;

struct PlainPoint {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct CopyPoint {
    x: i32,
    y: i32,
}

#[derive(Clone)] // 加上 Copy 会报 E0204：String 不是 Copy。
struct NamedPoint {
    x: i32,
    name: String,
}

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

    let plain1 = PlainPoint { x: 1, y: 2 };
    let plain2 = plain1; // 字段都是 i32，结构体没有 Copy，整体仍然 move。
    assert_eq!(plain2.x + plain2.y, 3);
    // println!("{}", plain1.x); // E0382：整体移动后，整数字段也不能再从原变量访问。
    println!("PlainPoint Move: sum={}", plain2.x + plain2.y);

    let copy1 = CopyPoint { x: 1, y: 2 };
    let copy2 = copy1;
    assert_eq!(copy1.x + copy2.y, 3);
    println!("CopyPoint Copy: original.x={}, copy.y={}", copy1.x, copy2.y);

    let named1 = NamedPoint {
        x: 1,
        name: String::from("A"),
    };
    let mut named2 = named1.clone(); // 显式 clone，字符串副本可以独立修改。
    named2.name.push('!');
    assert_eq!(named1.name, "A");
    assert_eq!(named2.name, "A!");
    let named3 = named1; // 只有 Clone，普通赋值仍然 move。
    assert_eq!(named3.x, 1);
    // println!("{}", named1.name); // E0382：不会自动调用 clone。
    println!(
        "NamedPoint Clone/Move: moved={}, cloned={}",
        named3.name, named2.name
    );

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
