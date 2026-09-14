#[derive(Debug)]
struct User {
    name: String,
}

fn lookup(id: u32) -> Option<User> {
    (id == 7).then(|| User {
        name: String::from("Ada"),
    })
}

enum State {
    Pending(User),
    Done { bytes: usize },
    Failed(String),
}

fn describe(state: &State) -> String {
    match state {
        State::Pending(user) => format!("pending {}", user.name),
        State::Done { bytes } => format!("done {bytes}"),
        State::Failed(reason) => format!("failed {reason}"),
    }
}

pub fn run() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len(); // Shadowing can change the binding's type.
    println!("Shadowed value: {spaces}");

    let block_value = {
        let base = 2;
        base * 3 // No semicolon: this block evaluates to 6.
    };
    println!("Block value: {block_value}");

    let Some(user) = lookup(7) else {
        println!("User not found");
        return;
    };

    let role = match user.name.as_str() {
        "Ada" => "administrator",
        _ => "member",
    };
    println!("Pattern match: {} is {role}", user.name);

    for state in [
        State::Pending(user),
        State::Done { bytes: 12 },
        State::Failed(String::from("timeout")),
    ] {
        println!("Enum state: {}", describe(&state));
    }

    let value = Some(String::from("kept by borrowing"));
    if let Some(name) = &value {
        println!("Borrowed pattern: {name}");
    }
    let _ = value; // Wildcard does not take ownership of this place.
    assert!(value.is_some());
    let _owned = value; // A named binding moves this non-Copy value.
                        // println!("{value:?}"); // E0382: value was moved into _owned.
}

#[cfg(test)]
mod tests {
    use super::{describe, lookup, State};

    #[test]
    fn lookup_exposes_both_option_states() {
        assert!(lookup(7).is_some());
        assert!(lookup(8).is_none());
    }

    #[test]
    fn describes_each_mutually_exclusive_state() {
        assert_eq!(describe(&State::Pending(lookup(7).unwrap())), "pending Ada");
        assert_eq!(describe(&State::Done { bytes: 0 }), "done 0");
        assert_eq!(describe(&State::Failed("timeout".into())), "failed timeout");
    }
}
