// 问题：怎样提交一个独立任务，并取得它的结果？
// 输入与状态：spawn 接收返回 42 的 Future，交回 JoinHandle，再等待这个句柄。
// 结果证据：句柄交付 42；测试用信号确认任务不必等调用者 await 句柄才推进。
// 改一处：删除 handle.await，观察主任务不再确认子任务完成。
// 边界：spawn 创建 Task，不是为每个任务创建 OS 线程；示例不承诺调度顺序。
// 运行：cargo run --locked --example 08-spawn-task

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    let handle = tokio::spawn(async { 42_u8 });
    let value = handle.await.expect("task panicked");
    assert_eq!(value, 42);
    println!("Task result: {value}");
}

#[cfg(test)]
mod tests {
    #[tokio::test(flavor = "current_thread")]
    async fn task_progresses_before_its_handle_is_awaited() {
        let (started, observed) = tokio::sync::oneshot::channel();
        let handle = tokio::spawn(async move {
            started.send(()).unwrap();
            42_u8
        });
        observed.await.unwrap(); // 此处尚未 await handle，任务已经发出通知。
        assert_eq!(handle.await.unwrap(), 42);
    }
}
