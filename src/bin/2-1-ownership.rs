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
}
