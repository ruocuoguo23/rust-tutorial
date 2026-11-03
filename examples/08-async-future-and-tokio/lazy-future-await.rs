// 问题：调用 async fn 后，函数体是否已经执行？
// 输入与状态：先创建 answer 的 Future，再通过 await 推进它并取得 7。
// 结果证据：输出顺序是 1 → 2 → 3；测试另用计数核对未 poll 的 Future 不执行函数体。
// 改一处：将 future.await 改为 { drop(future); 7 }，观察“2. 执行 answer”不再出现。
// 边界：这里只说明函数体的惰性；调用前的实参仍会求值，await 不保证每次挂起。
// 运行：cargo run --locked --example 08-lazy-future-await

async fn answer() -> u32 {
    println!("2. 执行 answer");
    7
}

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    let future = answer();
    println!("1. 只创建了 Future");
    let value = future.await;
    assert_eq!(value, 7);
    println!("3. 得到 {value}");
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    async fn counted_value(calls: &Cell<u8>) -> u8 {
        calls.set(calls.get() + 1);
        tokio::task::yield_now().await;
        7
    }

    #[tokio::test]
    async fn async_body_runs_only_when_polled() {
        let calls = Cell::new(0);
        let future = counted_value(&calls);
        assert_eq!(calls.get(), 0);
        assert_eq!(future.await, 7);
        assert_eq!(calls.get(), 1);
        drop(counted_value(&calls));
        assert_eq!(calls.get(), 1); // 未 poll 的 Future 不进入函数体。
    }
}
