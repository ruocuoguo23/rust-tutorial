# 06 异步 Future 与 Tokio：按示例学习

先读文件开头的问题与输入，预测状态变化，再单独运行并对照输出或断言。最后按“改一处”提示修改代码，观察结果，复习时从表格中的问题回想机制。注释中的编译失败或 panic 对照不会在正常运行中执行。

整章入口保持原命令，按 P0 → P1 顺序调用同一份示例代码：

```sh
cargo run --locked --bin 06-async-future-and-tokio
```

| 优先级 | 笔记小节 | 示例文件 | 运行命令 | 观察重点 |
| --- | --- | --- | --- | --- |
| P0 | [P0 Future与async/await](../../../blocksmith-codex/backend-engineering/rust-language/06-async-future-and-tokio.md#p0-future与asyncawait) | [lazy-future-await.rs](lazy-future-await.rs) | `cargo run --locked --example 06-lazy-future-await` | 创建 Future、开始 poll、完成 await，calls 分别是多少？ |
| P0 | [P0 Executor与Tokio](../../../blocksmith-codex/backend-engineering/rust-language/06-async-future-and-tokio.md#p0-executor与tokio) | [spawn-blocking.rs](spawn-blocking.rs) | `cargo run --locked --example 06-spawn-blocking` | 同步计算卸载到阻塞池；await 结果不占住异步线程；信号测试验证 B 在工作结束前推进。 |
| P1 | [P1 Poll与Waker](../../../blocksmith-codex/backend-engineering/rust-language/06-async-future-and-tokio.md#p1-poll与waker) | [poll-waker.rs](poll-waker.rs) | `cargo run --locked --example 06-poll-waker` | Pending 之后是谁请求下一次 poll？ |
| P1 | [P1 Pin与Unpin](../../../blocksmith-codex/backend-engineering/rust-language/06-async-future-and-tokio.md#p1-pin与unpin) | [pin-unpin.rs](pin-unpin.rs) | `cargo run --locked --example 06-pin-unpin` | 移动的是指针句柄还是被 pin 的值？ |
| P1 | [场景：Future为什么不是Send](../../../blocksmith-codex/backend-engineering/rust-language/06-async-future-and-tokio.md#场景future为什么不是send) | [send-future.rs](send-future.rs) | `cargo run --locked --example 06-send-future` | 哪些状态跨 await 保存，Rc 是否仍在其中？ |
| P1 | [select! 的选择与取消](../../../blocksmith-codex/computer-science/04-language/rust/06-async-future-and-tokio.md#select-的选择与取消) | [select-cancellation.rs](select-cancellation.rs) | `cargo run --locked --example 06-select-cancellation` | 超时丢弃拥有的 Future 并释放局部资源；借用外部 Future 不销毁本体；已有副作用不回滚。 |
| P1 | [P1 Channel与取消安全](../../../blocksmith-codex/backend-engineering/rust-language/06-async-future-and-tokio.md#p1-channel与取消安全) | [channel-cancellation.rs](channel-cancellation.rs) | `cargo run --locked --example 06-channel-cancellation` | 取消 send 与 reserve，消息分别由谁拥有？ |
| P1 | [P1 Channel与取消安全](../../../blocksmith-codex/backend-engineering/rust-language/06-async-future-and-tokio.md#p1-channel与取消安全) | [detach-abort.rs](detach-abort.rs) | `cargo run --locked --example 06-detach-abort` | 丢弃句柄与 abort 有何区别，之前的副作用会保留吗？ |

每个异步示例使用独立的 `current_thread` runtime，章节入口同步依次调用，不嵌套 runtime。任务通过等待 `JoinHandle` 或 oneshot 确认完成，通过等待取消结果确认停止；没有依赖睡眠时间猜测调度。本章验证局部状态与取消效果，不模拟真实网络负载或多线程 runtime 性能。
