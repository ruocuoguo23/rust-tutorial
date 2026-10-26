// 问题：线程需要持有所有权，还是可以借用调用者的局部数据？
// 输入与状态：spawn 消费 String；scope 借用 [1, 2, 3] 并在返回前 join。
// 结果证据：长度为 5、和为 6，调用者还能读取原 Vec；测试也覆盖空输入。
// 改一处：把 scope 中的局部引用交给 spawn，观察借用不满足 static。
// 边界：move 只转移捕获值；捕获引用不会自动延长被引用对象的生命周期。
// 运行：cargo run --locked --example 05-thread-ownership-scope

use std::thread;

fn owned_thread_len(message: String) -> usize {
    // An owned String satisfies 'static without living for the whole program.
    thread::spawn(move || message.len()).join().unwrap()
}

fn scoped_sum(values: &[u32]) -> u32 {
    // scope waits for its workers, so borrowing the caller's local data is valid.
    // Counterexample: move copies the reference, not the referenced String.
    // let text = String::from("local");
    // let borrowed = &text;
    // thread::spawn(move || borrowed.len()).join().unwrap(); // E0597: text is not 'static.
    thread::scope(|scope| scope.spawn(|| values.iter().sum()).join().unwrap())
}

pub fn main() {
    println!("P0: spawn / move / scope / join");
    let length = owned_thread_len(String::from("hello"));
    let values = vec![1, 2, 3];
    let sum = scoped_sum(&values);
    assert_eq!((length, sum), (5, 6));
    assert_eq!(values, [1, 2, 3]);
    println!("Moved String length: {length}; borrowed sum: {sum}; owner still usable");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owned_values_can_be_processed_and_joined() {
        assert_eq!(owned_thread_len(String::from("hello")), 5);
        assert_eq!(owned_thread_len(String::new()), 0);
    }

    #[test]
    fn scoped_threads_borrow_without_consuming_the_owner() {
        let values = vec![1, 2, 3];
        assert_eq!(scoped_sum(&values), 6);
        assert_eq!(values, [1, 2, 3]);
        assert_eq!(scoped_sum(&[]), 0);
    }
}
