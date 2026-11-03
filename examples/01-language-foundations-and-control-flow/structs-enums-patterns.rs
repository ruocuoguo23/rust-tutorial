// 问题：Struct、Enum 与模式匹配如何表达不同状态？
// 输入与状态：lookup(7) 返回 User；match 选择角色并分别读取 Pending、Done、Failed；PokerSuit 有四种值。
// 结果证据：输出四种花色与三种状态；测试实际执行 let-else 的成功/提前返回路径和所有 State 分支。
// 改一处：把 show_user(7) 改为 show_user(8)，观察 let-else 提前返回。
// 边界：Enum 一次只处于一个变体；新增变体后必须补全穷尽匹配。
// 运行：cargo run --locked --example 01-structs-enums-patterns

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

#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

fn print_suit(card: PokerSuit) {
    println!("{card:?}");
}

fn show_user(id: u32) -> bool {
    let Some(user) = lookup(id) else {
        println!("User not found");
        return false;
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

    print_suit(PokerSuit::Clubs);
    print_suit(PokerSuit::Spades);
    print_suit(PokerSuit::Hearts);
    print_suit(PokerSuit::Diamonds);
    true
}

pub fn main() {
    assert!(show_user(7));
}

#[cfg(test)]
mod tests {
    use super::{describe, lookup, show_user, State};

    #[test]
    fn let_else_runs_both_success_and_early_return() {
        assert!(show_user(7));
        assert!(!show_user(8));
    }

    #[test]
    fn describes_each_mutually_exclusive_state() {
        assert_eq!(describe(&State::Pending(lookup(7).unwrap())), "pending Ada");
        assert_eq!(describe(&State::Done { bytes: 0 }), "done 0");
        assert_eq!(describe(&State::Failed("timeout".into())), "failed timeout");
    }
}
