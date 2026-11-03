# 07 并发与同步：按示例学习

[配套笔记](../../../blocksmith-codex/computer-science/04-language/rust/07-concurrency-and-synchronization.md) · [仓库索引](../../README.md)

先看一个 guard 的访问范围，再共享给两个线程。基础例之后再读内存序与综合应用；每个入口都可独立运行。

| 阶段 | 笔记小节 | 示例文件 | 运行命令 | 观察重点 |
| --- | --- | --- | --- | --- |
| 基础 | [线程与捕获](../../../blocksmith-codex/computer-science/04-language/rust/07-concurrency-and-synchronization.md#p0-线程与捕获) | [thread-ownership-scope.rs](thread-ownership-scope.rs) | `cargo run --locked --example 07-thread-ownership-scope` | spawn 移入 String；scope 可借用局部数据。 |
| 基础 | [Send 与 Sync](../../../blocksmith-codex/computer-science/04-language/rust/07-concurrency-and-synchronization.md#p0-send与sync) | [send-sync.rs](send-sync.rs) | `cargo run --locked --example 07-send-sync` | 能力要求与 Arc 跨线程移动；复杂类型组合留在测试。 |
| 基础 | [单次 Mutex 访问](../../../blocksmith-codex/computer-science/04-language/rust/07-concurrency-and-synchronization.md#单次访问guard-离开作用域后解锁) | [mutex-basics.rs](mutex-basics.rs) | `cargo run --locked --example 07-mutex-basics` | 先得到 guard，再看作用域退出后的重新加锁。 |
| 基础 | [Arc 与 Mutex](../../../blocksmith-codex/computer-science/04-language/rust/07-concurrency-and-synchronization.md#p0-arc与mutex) | [mutex-counter.rs](mutex-counter.rs) | `cargo run --locked --example 07-mutex-counter` | 两个 worker 各加一次，join 后为 2。 |
| 基础 | [Channel](../../../blocksmith-codex/computer-science/04-language/rust/07-concurrency-and-synchronization.md#p0-channel) | [bounded-channel.rs](bounded-channel.rs) | `cargo run --locked --example 07-bounded-channel` | 容量、ownership、排空与关闭围绕一条消息队列展开。 |
| 基础 | [RwLock](../../../blocksmith-codex/computer-science/04-language/rust/07-concurrency-and-synchronization.md#p1-rwlock与poisoning) | [rwlock.rs](rwlock.rs) | `cargo run --locked --example 07-rwlock` | 先读、再写，同一份值从 7 变 8。 |
| 深入 | [Atomic](../../../blocksmith-codex/computer-science/04-language/rust/07-concurrency-and-synchronization.md#p1-atomic) | [atomic-updates.rs](atomic-updates.rs) | `cargo run --locked --example 07-atomic-updates` | RMW 与分开的 load/store 是不同动作粒度。 |
| 深入 | [内存序](../../../blocksmith-codex/computer-science/04-language/rust/07-concurrency-and-synchronization.md#p1-原子性与内存序) | [memory-ordering.rs](memory-ordering.rs) | `cargo run --locked --example 07-memory-ordering` | 分别观察发布关系与 SeqCst 全序；协调工具用于验证。 |
| 综合 | [完整临界区](../../../blocksmith-codex/computer-science/04-language/rust/07-concurrency-and-synchronization.md#p0-arc与mutex) | [mutex-guard-invariants.rs](mutex-guard-invariants.rs) | `cargo run --locked --example 07-mutex-guard-invariants` | 复习计数与 guard，再看余额转移的不变量。 |
| 综合 | [逻辑竞态](../../../blocksmith-codex/computer-science/04-language/rust/07-concurrency-and-synchronization.md#p0-数据竞争逻辑竞态与死锁) | [logical-race.rs](logical-race.rs) | `cargo run --locked --example 07-logical-race` | Barrier 固定交错，说明分开加锁仍会丢更新。 |
| 综合 | [锁顺序](../../../blocksmith-codex/computer-science/04-language/rust/07-concurrency-and-synchronization.md#p0-数据竞争逻辑竞态与死锁) | [lock-order.rs](lock-order.rs) | `cargo run --locked --example 07-lock-order` | 两把锁按一致顺序获取；死锁对照不执行。 |
| 综合 | [跨线程 Store](../../../blocksmith-codex/computer-science/04-language/rust/07-concurrency-and-synchronization.md#完整示例跨线程共享-hashmap-store) | [shared-store.rs](shared-store.rs) | `cargo run --locked --example 07-shared-store` | 在 Trait、Arc、RwLock 基础上组合一次写入与读取。 |

整章按上表调用同源代码：

```sh
cargo run --locked --bin 07-concurrency-and-synchronization
```

原 `07-send-sync` 命令保留并聚焦能力要求；其中 Store 实现及三项测试迁入 `07-shared-store`。原 `07-mutex-guard-invariants` 保留为综合回顾，两个新锁入口先建立直觉。

类型约束检查只说明编译条件；RefCell/Mutex 的嵌套能力对照在 send-sync 测试中。Store 的 join 建立读写阶段，get 返回独立 String，expect 不实现中毒恢复。

并发结果通过 join、Barrier 和明确状态观察；普通运行不穷尽弱内存行为，也不证明公平性或任意程序的活性。拒编和死锁对照只在注释中，不在正常路径执行。
