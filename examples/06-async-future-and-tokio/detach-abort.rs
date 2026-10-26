// 问题：丢弃 JoinHandle 和调用 abort，对已启动的任务分别有什么影响？
// 输入与状态：一个任务发回 42 后结束；另一个任务先记录一次副作用，再 pending 等待取消。
// 结果证据：oneshot 确认 detached 任务仍完成；等待 abort 的 JoinHandle 后断言副作用计数仍为 1。
// 改一处：在 drop(handle) 后停止等待 finished，观察失去完成确认，不能再据此声称任务已完成。
// 边界：本例用确认信号排除时序猜测；abort 请求取消并非同步立即停止，也不会撤销此前副作用。
// 运行：cargo run --locked --example 06-detach-abort

use std::future;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::oneshot;

pub async fn detached_task_completes() -> u8 {
    let (finish, finished) = oneshot::channel();
    let handle = tokio::spawn(async move {
        finish.send(42).unwrap();
    });
    drop(handle); // Dropping JoinHandle detaches; it does not abort the task.
    finished.await.unwrap() // Completion acknowledgement, without timing assumptions.
}

pub async fn aborted_task_keeps_prior_effects() -> usize {
    let effects = Arc::new(AtomicUsize::new(0));
    let observed = Arc::clone(&effects);
    let (started, ready) = oneshot::channel();
    let handle = tokio::spawn(async move {
        observed.fetch_add(1, Ordering::Relaxed);
        started.send(()).unwrap();
        future::pending::<()>().await;
        observed.fetch_add(1, Ordering::Relaxed);
    });
    ready.await.unwrap(); // Ensure the first effect happened before requesting cancellation.
    handle.abort();
    assert!(handle.await.unwrap_err().is_cancelled());
    effects.load(Ordering::Relaxed) // Cancellation stops the next step, but does not roll back 1.
}

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    println!("P1: Detach / abort");
    println!("Detached task result: {}", detached_task_completes().await);
    println!(
        "Prior effects after abort: {}",
        aborted_task_keeps_prior_effects().await
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn detach_and_abort_have_different_effects() {
        assert_eq!(detached_task_completes().await, 42);
        assert_eq!(aborted_task_keeps_prior_effects().await, 1);
    }
}
