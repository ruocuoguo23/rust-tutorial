// 问题：创建 Future 是否立即执行 async 函数体？
// 输入与状态：calls 从 0 开始；创建 Future 后不变，await 驱动执行后变为 1。
// 结果证据：断言创建时 calls=0，完成时 result=7、calls=1；测试还检查丢弃未 poll 的 Future。
// 改一处：把 future.await 改为 { drop(future); 7 }，观察 calls 保持 0，完成执行的断言会失败。
// 边界：这是协作式异步示例；await 不保证每次都挂起，也不表示启动了线程。
// 运行：cargo run --locked --example 06-lazy-future-await

use std::cell::Cell;

async fn lazy_value(calls: &Cell<u8>) -> u8 {
    calls.set(calls.get() + 1);
    tokio::task::yield_now().await;
    7
}

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    println!("P0: Future / async / await");
    let calls = Cell::new(0);
    let future = lazy_value(&calls);
    assert_eq!(calls.get(), 0);
    println!("Constructed Future: calls={}", calls.get());
    let value = future.await;
    assert_eq!(value, 7);
    assert_eq!(calls.get(), 1);
    println!("After await: calls={}, result={value}", calls.get());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn async_body_runs_only_when_polled() {
        let calls = Cell::new(0);
        let future = lazy_value(&calls);
        assert_eq!(calls.get(), 0);
        assert_eq!(future.await, 7);
        assert_eq!(calls.get(), 1);
        drop(lazy_value(&calls));
        assert_eq!(calls.get(), 1); // Never-polled Future caused no body side effect.
    }
}
