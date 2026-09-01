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

fn main() {
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
