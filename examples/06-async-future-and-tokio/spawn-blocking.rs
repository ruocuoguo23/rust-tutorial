// 问题：同步阻塞、异步等待与 CPU 计算卸载有什么区别？
// 输入与状态：current_thread 提交同步求和到阻塞池，另一个异步任务继续执行，再取回结果。
// 结果证据：求和为 500000500000；测试用信号保证 B 在阻塞工作结束前执行。
// 改一处：将计算直接放进 tokio::spawn(async { ... })，它仍占用异步执行线程，不会自动卸载。
// 边界：示例不衡量计算耗时；大量 CPU 工作要限制并发；阻塞工作开始后不能靠 abort 停止。
// 运行：cargo run --locked --example 06-spawn-blocking

fn calculate_sum(limit: u64) -> u64 {
    (1..=limit).sum()
}

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    println!("P0: Executor / Tokio / spawn / spawn_blocking");
    let result = tokio::spawn(async { 6 * 7 }).await.unwrap();
    assert_eq!(result, 42);

    let limit = 1_000_000;
    let work = tokio::task::spawn_blocking(move || calculate_sum(limit));
    let other = tokio::spawn(async {
        println!("B can run on the async runtime");
    });
    let sum = work.await.expect("calculation panicked");
    other.await.expect("B panicked");
    assert_eq!(sum, 500_000_500_000);
    println!("spawn: {result}; spawn_blocking sum: {sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_handles_empty_and_nonempty_ranges() {
        assert_eq!(calculate_sum(0), 0);
        assert_eq!(calculate_sum(10), 55);
        assert_eq!(calculate_sum(1_000_000), 500_000_500_000);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn async_task_progresses_before_blocking_work_is_released() {
        let (release, wait) = std::sync::mpsc::channel();
        let (started_tx, started_rx) = tokio::sync::oneshot::channel();
        let work = tokio::task::spawn_blocking(move || {
            started_tx.send(()).expect("observer dropped");
            // 只在阻塞线程使用阻塞 recv，保证工作在 B 发送之前不会结束。
            wait.recv().expect("release sender dropped");
            calculate_sum(10)
        });
        started_rx.await.expect("worker did not start");
        let other = tokio::spawn(async move {
            release.send(()).expect("worker dropped receiver");
        });
        assert_eq!(work.await.expect("calculation panicked"), 55);
        other.await.expect("B panicked");
    }
}
