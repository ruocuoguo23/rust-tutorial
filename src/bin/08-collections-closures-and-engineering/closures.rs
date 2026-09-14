use std::cell::Cell;

fn call_once<F, T>(operation: F) -> T
where
    F: FnOnce() -> T,
{
    operation()
}

pub fn run() {
    let mut words = vec![String::from("rust"), String::from("go")];

    let lengths: Vec<usize> = words.iter().map(String::len).collect();
    println!("Lengths from iter: {lengths:?}");

    for word in words.iter_mut() {
        word.push('!');
    }
    println!("After iter_mut: {words:?}");

    let label = String::from("owned by an Fn closure");
    let length = move || label.len(); // Owning capture still permits repeated read-only calls.
    assert_eq!(length(), length());
    println!("move + Fn: repeated length={}", length());

    let mut calls = 0;
    let mut increment = || {
        calls += 1;
        calls
    };
    assert_eq!((increment(), increment()), (1, 2));
    println!("FnMut: calls={calls}");

    let label = String::from("consumed by FnOnce");
    let consume = move || label; // Moving out of the capture makes this closure FnOnce-only.
    println!("FnOnce returned ownership: {}", call_once(consume));
    // consume(); // E0382: call_once already consumed the closure.

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

#[cfg(test)]
mod tests {
    use super::call_once;
    use std::cell::Cell;

    #[test]
    fn call_once_accepts_a_closure_that_moves_out_its_capture() {
        let value = String::from("owned");
        assert_eq!(call_once(move || value), "owned");
        let calls = Cell::new(0);
        call_once(|| calls.set(calls.get() + 1));
        assert_eq!(calls.get(), 1);
    }
}
