# 08 Async、Future 与 Tokio：按示例学习

先沿入门顺序运行，每个示例只观察一个新动作：创建 Future、等待结果、提交任务、收发消息、选择分支。能解释输入、状态变化和结果之后，再读类型约束、取消和内部机制。正文参考 [Tokio 官方教程](https://tokio.rs/tokio/tutorial)的递进方法，使用独立本地程序，不依赖 Mini-Redis 服务。

[配套笔记](../../../blocksmith-codex/computer-science/04-language/rust/08-async-future-and-tokio.md) · [仓库索引](../../README.md)

## 入门：一次理解一个动作

各例使用 `#[tokio::main(flavor = "current_thread")]` 启动运行时；根依赖已经锁定 Tokio 1.53.1，启用 `macros / rt / sync / time`。先读完整例子，预测结果，再运行对应命令。

| 顺序 | 笔记小节 | 示例文件 | 运行命令 | 只观察什么 |
| --- | --- | --- | --- | --- |
| 1 | [Future 与 async/await](../../../blocksmith-codex/computer-science/04-language/rust/08-async-future-and-tokio.md#p0-future与asyncawait) | [lazy-future-await.rs](lazy-future-await.rs) | `cargo run --locked --example 08-lazy-future-await` | 创建时不执行函数体，await 后进入；输出 1 → 2 → 3。 |
| 2 | [顺序 await](../../../blocksmith-codex/computer-science/04-language/rust/08-async-future-and-tokio.md#顺序-await先等完一个再开始下一个) · [join!](../../../blocksmith-codex/computer-science/04-language/rust/08-async-future-and-tokio.md#join同时等待全部结果) | [join-futures.rs](join-futures.rs) | `cargo run --locked --example 08-join-futures` | 用同一函数比较顺序等待与共同推进；不以耗时或日志先后作证明。 |
| 3 | [spawn 与 JoinHandle](../../../blocksmith-codex/computer-science/04-language/rust/08-async-future-and-tokio.md#spawn提交独立任务) | [spawn-task.rs](spawn-task.rs) | `cargo run --locked --example 08-spawn-task` | 提交一个独立任务，再通过句柄得到 42。 |
| 4 | [mpsc 基础](../../../blocksmith-codex/computer-science/04-language/rust/08-async-future-and-tokio.md#mpsc先发送再接收) | [mpsc-basics.rs](mpsc-basics.rs) | `cargo run --locked --example 08-mpsc-basics` | 队列接收 7、8，最后发送端退出后排空，得到 [7, 8]。 |
| 5 | [oneshot](../../../blocksmith-codex/computer-science/04-language/rust/08-async-future-and-tokio.md#oneshot只交付一次结果) | [oneshot-basics.rs](oneshot-basics.rs) | `cargo run --locked --example 08-oneshot-basics` | send 一次值，receiver.await 得到这个结果；send 本身不需要 await。 |
| 6 | [select!](../../../blocksmith-codex/computer-science/04-language/rust/08-async-future-and-tokio.md#select只选择一个就绪分支) | [select-basics.rs](select-basics.rs) | `cargo run --locked --example 08-select-basics` | 左通道已就绪、右通道尚未就绪，处理左分支，不混入超时与析构。 |
| 7 | [阻塞工作与 spawn_blocking](../../../blocksmith-codex/computer-science/04-language/rust/08-async-future-and-tokio.md#阻塞工作与spawn_blocking) | [spawn-blocking.rs](spawn-blocking.rs) | `cargo run --locked --example 08-spawn-blocking` | 同步函数交给阻塞线程池，再等待结果。 |

## 深入：生命周期、类型约束与内部机制

先知道这些 API 怎样使用，再分别追问“任务怎样退出”“值能否跨等待保存”“谁继续推进 Future”。以下例子保留复杂状态或验证工具，不放进初次认识 API 的程序。

| 主题 | 笔记小节 | 示例文件 | 运行命令 | 观察重点 |
| --- | --- | --- | --- | --- |
| 任务退出 | [Future、JoinHandle 与退出](../../../blocksmith-codex/computer-science/04-language/rust/08-async-future-and-tokio.md#futurejoinhandle与任务退出) | [detach-abort.rs](detach-abort.rs) | `cargo run --locked --example 08-detach-abort` | drop 句柄与 abort 的区别；已发生修改不会回滚。 |
| Send | [Future 为什么不是 Send](../../../blocksmith-codex/computer-science/04-language/rust/08-async-future-and-tokio.md#场景future为什么不是send) | [send-future.rs](send-future.rs) | `cargo run --locked --example 08-send-future` | 哪些捕获或局部值保存在 Future 中；缩短 Rc 作用域的效果。 |
| poll/Waker | [Poll 与 Waker](../../../blocksmith-codex/computer-science/04-language/rust/08-async-future-and-tokio.md#p1-poll与waker) | [poll-waker.rs](poll-waker.rs) | `cargo run --locked --example 08-poll-waker` | 返回 Pending 后，是谁请求再 poll？ |
| Pin | [Pin 与 Unpin](../../../blocksmith-codex/computer-science/04-language/rust/08-async-future-and-tokio.md#p1-pin与unpin) | [pin-unpin.rs](pin-unpin.rs) | `cargo run --locked --example 08-pin-unpin` | 移动指针句柄与移动目标值的区别。 |
| 选择后的取消 | [select! 的选择与取消](../../../blocksmith-codex/computer-science/04-language/rust/08-async-future-and-tokio.md#select-的选择与取消) | [select-cancellation.rs](select-cancellation.rs) | `cargo run --locked --example 08-select-cancellation` | 拥有 Future 与借用 Future 的销毁边界；局部资源与外部副作用。 |
| 取消发送 | [Channel 与取消安全](../../../blocksmith-codex/computer-science/04-language/rust/08-async-future-and-tokio.md#p1-channel与取消安全) | [channel-cancellation.rs](channel-cancellation.rs) | `cargo run --locked --example 08-channel-cancellation` | send 与 reserve 等待被取消后，消息分别由谁拥有？ |

## 运行整章与验证范围

整章同步入口按以上顺序调用同一份源码，并打印“入门”“深入”分隔；各异步示例独立创建并结束 runtime，避免嵌套 runtime，也不跨例子留下未确认的教学任务。

```sh
cargo run --locked --bin 08-async-future-and-tokio
```

主例使用简短输出和结果断言；Cell 计数、oneshot 握手、DropProbe 等协调或观测手段留在测试或深入例中。测试确认 Future 未 poll 时不执行、join 能推进两分支、spawn 无需先 await 句柄、通道满与关闭、阻塞卸载，以及取消后的状态。它们不依赖固定 sleep 猜测另一个任务的进度。

注释中的拒编、panic 或死锁对照不会在正常路径执行。每次运行和测试只支持所覆盖的状态；没有真实网络 I/O，不测多线程 runtime、吞吐或公平性，也不证明所有 unsafe 路径或用户掌握程度。
