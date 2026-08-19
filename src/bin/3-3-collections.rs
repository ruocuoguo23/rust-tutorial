use std::collections::HashMap;

fn main() {
    let mut values = vec![10, 20, 30];

    {
        let first = &values[0];
        println!("Borrowed first value: {first}");
    }
    values.push(40); // The element borrow ended before Vec was mutated.
    println!("Vec after push: {values:?}");

    // The following order cannot compile because push needs &mut Vec while first is live:
    // let first = &values[0];
    // values.push(50); // E0502
    // println!("{first}");

    let mut counts = HashMap::new();
    for word in ["rust", "go", "rust"] {
        *counts.entry(word).or_insert(0) += 1;
    }
    println!("rust count: {}", counts["rust"]);
    println!("go count: {}", counts["go"]);
}
