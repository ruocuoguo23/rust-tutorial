mod closures;
mod collections;

use std::borrow::Cow;

fn normalize(text: &str) -> Cow<'_, str> {
    if text.bytes().any(|byte| byte.is_ascii_uppercase()) {
        Cow::Owned(text.to_ascii_lowercase())
    } else {
        Cow::Borrowed(text)
    }
}

macro_rules! twice {
    ($expression:expr) => {{
        let value = $expression; // Evaluate side effects once, then use the integer twice.
        value + value
    }};
}

fn main() {
    println!("P1 supplement: String / str / Vec / slice / HashMap");
    collections::run();
    println!("P1 supplement: Cow (Clone-on-Write)");
    assert!(matches!(normalize("rust"), Cow::Borrowed(_)));
    assert!(matches!(normalize("Rust"), Cow::Owned(_)));
    println!(
        "normalize: rust -> Borrowed; Rust -> Owned({})",
        normalize("Rust")
    );
    let mut text = Cow::Borrowed("rust");
    text.to_mut().push('!');
    assert!(matches!(text, Cow::Owned(_)));
    println!("First write changes Borrowed to Owned: {text}");

    println!("P1 supplement: Fn / FnMut / FnOnce / Iterator");
    closures::run();
    println!("P1 supplement: Macro / module / Cargo");
    let mut calls = 0;
    let doubled = twice!({
        calls += 1;
        3
    });
    assert_eq!((doubled, calls), (6, 1));
    println!("twice!: value={doubled}, expression evaluations={calls}");
    // This binary is one crate; collections and closures are modules, not separate crates.
    // See README for cargo tree -e features, validation commands, and evidence boundaries.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalization_borrows_unchanged_input_and_owns_ascii_changes() {
        for text in ["rust", "", "世界", "É"] {
            assert!(matches!(normalize(text), Cow::Borrowed(_)));
            assert_eq!(normalize(text), text); // This is ASCII case folding, not Unicode normalization.
        }
        for (input, expected) in [("Rust", "rust"), ("RUST", "rust"), ("世界A", "世界a")] {
            let normalized = normalize(input);
            assert!(matches!(normalized, Cow::Owned(_)));
            assert_eq!(normalized, expected);
        }
    }

    #[test]
    fn macro_evaluates_its_argument_once() {
        let mut calls = 0;
        assert_eq!(
            twice!({
                calls += 1;
                3
            }),
            6
        );
        assert_eq!(calls, 1);
    }
}
