#[derive(Debug)]
struct User {
    name: String,
}

fn lookup(id: u32) -> Option<User> {
    (id == 7).then(|| User {
        name: String::from("Ada"),
    })
}

fn main() {
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
}

#[cfg(test)]
mod tests {
    use super::lookup;

    #[test]
    fn lookup_exposes_both_option_states() {
        assert!(lookup(7).is_some());
        assert!(lookup(8).is_none());
    }
}
