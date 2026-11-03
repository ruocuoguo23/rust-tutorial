// 问题：怎样把会占用线程的同步计算交给阻塞线程池？
// 输入与状态：提交同步求和函数，当前任务 await 返回的句柄，最后取得计算结果。
// 结果证据：求和为 500000500000；测试另用握手确认阻塞工作未结束时异步任务仍可推进。
// 改一处：把同步计算直接放进 tokio::spawn(async { ... })，它不会因此转到阻塞线程池。
// 边界：本例不测吞吐或计算耗时；大量 CPU 工作需限制并行度，已开始的阻塞工作不能靠 abort 强停。
// 运行：cargo run --locked --example 08-spawn-blocking

fn calculate_sum(limit: u64) -> u64 {
    (1..=limit).sum()
}

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    let work = tokio::task::spawn_blocking(|| calculate_sum(1_000_000));
    let sum = work.await.expect("calculation panicked");
    assert_eq!(sum, 500_000_500_000);
    println!("Blocking calculation result: {sum}");
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
