fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

fn edit(text: &mut String) {
    let view: &str = &*text;
    println!("Shared borrow: {view}");
    text.push('!'); // NLL ends view's borrow after its last use.
}

pub fn reborrow_and_static() {
    let mut text = String::from("rust");
    let original = &mut text;
    let short = &mut *original;
    short.push('?');
    original.push('.'); // The shorter reborrow has ended.
    assert_eq!(text, "rust?.");

    let mut sizes = vec![1];
    sizes.push(sizes.len()); // This implicit mutable borrow supports two-phase borrowing.
    assert_eq!(sizes, [1, 1]);

    fn requires_static<T: 'static>(_: &T) {}
    requires_static(&text); // String contains no short-lived references.
    println!("Reborrow: {text}; two-phase borrow: {sizes:?}; owned String: 'static");
    drop(text); // T: 'static does not require this value to live forever.
}

fn no_dangle() -> String {
    String::from("owned value")
}

pub fn run() {
    let mut text = String::from("rust");
    edit(&mut text);
    println!("After edit: {text}");
    println!("Longest: {}", longest(&text, "go"));
    println!("Owned return: {}", no_dangle());

    // This cannot compile because the returned reference would outlive local:
    // fn dangling<'a>() -> &'a str {
    //     let local = String::from("temporary");
    //     &local // E0515
    // }
}

#[cfg(test)]
mod tests {
    use super::{edit, longest};

    #[test]
    fn returns_a_reference_from_the_inputs() {
        assert_eq!(longest("rust", "go"), "rust");
        assert_eq!(longest("go", "rust"), "rust");
        assert_eq!(longest("ab", "cd"), "ab");
        assert_eq!(longest("", ""), "");
    }

    #[test]
    fn shared_borrow_ends_before_mutation() {
        let mut text = String::from("rust");
        edit(&mut text);
        assert_eq!(text, "rust!");
    }
}
