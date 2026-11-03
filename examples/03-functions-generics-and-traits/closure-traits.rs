// 问题：学完三个独立闭包后，怎样综合判断 Fn、FnMut、FnOnce 的能力？
// 输入与状态：move 闭包反复读取拥有的 String，另一个闭包更新计数，第三个闭包把 String 移出。
// 结果证据：重复调用 Fn 得到同一长度，FnMut 两次返回 1、2，FnOnce 把 String 所有权交给调用方。
// 改一处：取消注释 consume() 的第二次调用，观察 FnOnce-only 闭包已被消费。
// 边界：move 捕获也可以实现 Fn；FnOnce 表示按值调用能力，不代表所有实现者只能调用一次。
// 运行：cargo run --locked --example 03-closure-traits

fn call_once<F, T>(operation: F) -> T
where
    F: FnOnce() -> T,
{
    operation()
}

pub fn main() {
    println!("P1: Fn / FnMut / FnOnce");
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
