fn call_once<F>(operation: F)
where
    F: FnOnce(),
{
    operation();
}

fn main() {
    let mut words = vec![String::from("rust"), String::from("go")];

    let lengths: Vec<usize> = words.iter().map(String::len).collect();
    println!("Lengths from iter: {lengths:?}");

    for word in words.iter_mut() {
        word.push('!');
    }
    println!("After iter_mut: {words:?}");

    let label = String::from("consumed by FnOnce");
    call_once(move || println!("Closure capture: {label}"));

    let upper: Vec<String> = words.into_iter().map(|word| word.to_uppercase()).collect();
    println!("Owned items from into_iter: {upper:?}");
    // println!("{words:?}"); // E0382: into_iter moved words.
}
