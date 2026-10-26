# 05：跨线程约束与同步

先区分“值能跨线程”和“共享状态如何同步”，再区分原子操作与内存序。整章按 P0 → P1 运行；并发输出通过 join、Barrier 和明确的读写阶段保持可观察。

| 优先级 | 笔记小节 | 示例文件 | 运行命令 | 观察重点 |
| --- | --- | --- | --- | --- |
| P0 | [线程与捕获](../../../blocksmith-codex/backend-engineering/rust-language/05-concurrency-and-synchronization.md#p0-线程与捕获) | [thread-ownership-scope.rs](thread-ownership-scope.rs) | `cargo run --locked --example 05-thread-ownership-scope` | spawn 持有 String；scope 借用局部 Vec 并等待线程结束。 |
| P0 | [Send 与 Sync](../../../blocksmith-codex/backend-engineering/rust-language/05-concurrency-and-synchronization.md#p0-send与sync) | [send-sync.rs](send-sync.rs) | `cargo run --locked --example 05-send-sync` | Store: Send + Sync 声明约束，RwLock 同步读写，Arc 共享实例；保留 RefCell／Mutex 类型边界。 |
| P0 | [Arc 与 Mutex](../../../blocksmith-codex/backend-engineering/rust-language/05-concurrency-and-synchronization.md#p0-arc与mutex) | [mutex-guard-invariants.rs](mutex-guard-invariants.rs) | `cargo run --locked --example 05-mutex-guard-invariants` | 一个 guard 覆盖整个计数或余额转移，drop guard 后解锁。 |
| P0 | [Channel](../../../blocksmith-codex/backend-engineering/rust-language/05-concurrency-and-synchronization.md#p0-channel) | [bounded-channel.rs](bounded-channel.rs) | `cargo run --locked --example 05-bounded-channel` | 消息 ownership 转移；先 drain 再 join，断开时错误返回未发送消息。 |
| P0 | [数据竞争、逻辑竞态与死锁](../../../blocksmith-codex/backend-engineering/rust-language/05-concurrency-and-synchronization.md#p0-数据竞争逻辑竞态与死锁) | [logical-race.rs](logical-race.rs) | `cargo run --locked --example 05-logical-race` | Barrier 让两个 worker 都读到 0；分别加锁读写仍丢更新。 |
| P0 | [数据竞争、逻辑竞态与死锁](../../../blocksmith-codex/backend-engineering/rust-language/05-concurrency-and-synchronization.md#p0-数据竞争逻辑竞态与死锁) | [lock-order.rs](lock-order.rs) | `cargo run --locked --example 05-lock-order` | 双方按相同顺序获取两把锁；相反顺序的死锁代码只作注释对照。 |
| P1 | [RwLock 与 poisoning](../../../blocksmith-codex/backend-engineering/rust-language/05-concurrency-and-synchronization.md#p1-rwlock与poisoning) | [rwlock.rs](rwlock.rs) | `cargo run --locked --example 05-rwlock` | 两个 reader 完成后再写，观察 [7, 7] → 8。 |
| P1 | [Atomic](../../../blocksmith-codex/backend-engineering/rust-language/05-concurrency-and-synchronization.md#p1-atomic)、[SeqCst 仍可能丢更新](../../../blocksmith-codex/backend-engineering/rust-language/05-concurrency-and-synchronization.md#seqcst仍可能丢更新) | [atomic-updates.rs](atomic-updates.rs) | `cargo run --locked --example 05-atomic-updates` | Relaxed fetch_add 保留全部增量；SeqCst load + store 仍丢更新。 |
| P1 | [原子性与内存序](../../../blocksmith-codex/backend-engineering/rust-language/05-concurrency-and-synchronization.md#p1-原子性与内存序) | [memory-ordering.rs](memory-ordering.rs) | `cargo run --locked --example 05-memory-ordering` | Acquire 观察 Release 发布的 payload；SeqCst 排除双方都读到 false。 |

整章运行：

```sh
cargo run --locked --bin 05-concurrency-and-synchronization
```

每个文件开头包含“改一处”的观察点。故意死锁的代码只分析等待关系，不放进正常运行路径。一次执行和断言不能穷尽弱内存行为，也不能证明公平性或任意程序的活性；弱化 ordering 后偶尔成功不代表仍有原来的保证。
