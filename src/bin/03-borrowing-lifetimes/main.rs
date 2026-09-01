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

    let short: &mut String = &mut *text; // Reborrow for a shorter lifetime.
    short.push('?');
    text.push('.'); // short is no longer used, so the original reference is usable again.
}

fn no_dangle() -> String {
    String::from("owned value")
}

fn main() {
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
    }

    #[test]
    fn reborrow_ends_before_the_original_reference_is_reused() {
        let mut text = String::from("rust");
        edit(&mut text);
        assert_eq!(text, "rust!?.");
    }
}
