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

pub fn run() {
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

pub fn partial_move_and_take() {
    struct User {
        name: String,
        age: u8,
    }
    let user = User {
        name: String::from("Ada"),
        age: 20,
    };
    let name = user.name;
    println!("Partial move: name={name}, remaining age={}", user.age);
    // let whole = user; // E0382: name was moved; user is no longer a complete value.

    let mut text = name;
    let taken = std::mem::take(&mut text);
    assert!(text.is_empty());
    let previous = std::mem::replace(&mut text, String::from("Grace"));
    assert!(previous.is_empty());
    println!("take: old={taken}; replace: current={text}");

    let mut pending = Some(taken);
    let delivered = pending.take();
    assert!(pending.is_none());
    println!("Option::take: delivered={delivered:?}, remaining={pending:?}");
    // take/replace leave a valid value; its business meaning still needs a contract.
}
