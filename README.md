# rust-tutorial

通过独立、可运行的 binary 学习 Rust 语言语义。目录对应 [Rust 学习地图](../blocksmith-codex/backend-engineering/rust-language/rust-semantics-ownership-types-and-standard-library.md)的「七条主线 + 一篇补充」，章节名与笔记文件名一致。

保持 Rust 2021 Edition。01–05、07–08 的示例使用标准库；06 使用锁定的 Tokio 1.53.1，启用 `macros`、`rt`、`sync`，实际驱动 Future、任务和 channel。Tokio 是 package 依赖，首次构建需要获取锁定依赖。示例是泛化教学模型。

## 运行与阅读顺序

```bash
# 入门
cargo run --locked --bin 00-hello-world

# 每章都是独立运行入口，输出按 P0 → P1 排列
cargo run --locked --bin 01-ownership-and-borrowing
cargo run --locked --bin 06-async-future-and-tokio

# 格式、编译与行为分别验证
cargo fmt --check
cargo check --locked --bins
cargo test --locked --bins

# 查看 feature 如何启用依赖能力
cargo tree --locked -e features
```

首轮运行 01–07，先预测 P0 的输出与所有权变化；第二轮读各章 P1。08 全部为 P1 补充。P2 保留在笔记中按需深入，本仓库不实现生产执行器、无锁算法或复杂 pin projection。P0/P1/P2 是学习顺序，不是能力或性能保证。

## 章节与重点

| 章节与运行入口 | P0 核心机制 | P1 补充与边界 | 对应笔记 |
| --- | --- | --- | --- |
| [01-ownership-and-borrowing](src/bin/01-ownership-and-borrowing/main.rs) | Move/Copy/Clone、borrow、NLL、lifetime | partial move、take/replace、reborrow、`T: 'static` | [Ownership](../blocksmith-codex/backend-engineering/rust-language/01-ownership-and-borrowing.md) |
| [02-type-system-and-error-handling](src/bin/02-type-system-and-error-handling/main.rs) | 表达式、Struct/Enum、模式、Option/Result/`?` | 区分解析错误与业务策略，保留 error source | [Type System](../blocksmith-codex/backend-engineering/rust-language/02-type-system-and-error-handling.md) |
| [03-traits-generics-and-dispatch](src/bin/03-traits-generics-and-dispatch/main.rs) | trait bound、Associated Type、`impl Trait`、`&dyn Trait` | newtype、From/Into、可失败转换、自定义相等语义 | [Trait](../blocksmith-codex/backend-engineering/rust-language/03-traits-generics-and-dispatch.md) |
| [04-memory-and-smart-pointers](src/bin/04-memory-and-smart-pointers/main.rs) | Box、Rc/Weak/Arc、RefCell、Drop/RAII、Deref | Cell 的非 Copy 值替换、DST/slice | [Memory](../blocksmith-codex/backend-engineering/rust-language/04-memory-and-smart-pointers.md) |
| [05-concurrency-and-synchronization](src/bin/05-concurrency-and-synchronization/main.rs) | spawn/move/scope/join、Send/Sync、Guard、多字段不变量、有界 channel、丢更新与锁顺序 | Arc/RwLock、Relaxed fetch_add、Acquire/Release 发布、SeqCst 全序与丢更新反例 | [Concurrency](../blocksmith-codex/backend-engineering/rust-language/05-concurrency-and-synchronization.md) |
| [06-async-future-and-tokio](src/bin/06-async-future-and-tokio/main.rs) | 惰性 Future、await、Tokio spawn/spawn_blocking | Poll/Waker、Pin/Unpin、非 Send 状态、有界 channel、取消与 detach/abort | [Async](../blocksmith-codex/backend-engineering/rust-language/06-async-future-and-tokio.md) |
| [07-unsafe-and-ffi](src/bin/07-unsafe-and-ffi/main.rs) | 裸指针、不变量、safe wrapper、safe API 对照 | opaque handle 的唯一释放责任与 null 协议 | [Unsafe](../blocksmith-codex/backend-engineering/rust-language/07-unsafe-and-ffi.md) |
| [08-collections-closures-and-engineering](src/bin/08-collections-closures-and-engineering/main.rs) | 补充篇，不进入首轮 P0 | UTF-8、Vec/slice、HashMap 排序、Cow、Fn/FnMut/FnOnce、惰性迭代器、宏、Cargo 验证 | [补充](../blocksmith-codex/backend-engineering/rust-language/08-collections-closures-and-engineering.md) |

每个 `src/bin/<chapter>/main.rs` 是该章唯一运行入口。同目录的 `.rs` 是按主题拆分的模块；例如 [ownership.rs](src/bin/01-ownership-and-borrowing/ownership.rs) 与 [borrowing.rs](src/bin/01-ownership-and-borrowing/borrowing.rs) 共同组成第 01 章。默认 `src/main.rs` 和 00 入门示例保留。

## 旧章节迁移

旧 binary 名已由新章节名替代，运行命令需要改用上表。原示例按知识归属合并，补充代码放在同一章运行入口。

| 旧入口 | 新位置 |
| --- | --- |
| `01-values-types-patterns` | [02 / patterns.rs](src/bin/02-type-system-and-error-handling/patterns.rs) |
| `02-ownership-move-copy-clone` | [01 / ownership.rs](src/bin/01-ownership-and-borrowing/ownership.rs) |
| `03-borrowing-lifetimes` | [01 / borrowing.rs](src/bin/01-ownership-and-borrowing/borrowing.rs) |
| `04-strings-slices-collections` | [08 / collections.rs](src/bin/08-collections-closures-and-engineering/collections.rs) |
| `05-enums-option-result` | [02 / errors.rs](src/bin/02-type-system-and-error-handling/errors.rs) |
| `06-generics-traits` | [03 / main.rs](src/bin/03-traits-generics-and-dispatch/main.rs) |
| `07-smart-pointers` | [04 / main.rs](src/bin/04-memory-and-smart-pointers/main.rs) |
| `08-closures-iterators` | [08 / closures.rs](src/bin/08-collections-closures-and-engineering/closures.rs) |
| `09-send-threads-async` | [05 / threads](src/bin/05-concurrency-and-synchronization/main.rs)、[06 / Send Future](src/bin/06-async-future-and-tokio/send.rs) |
| `10-unsafe-boundaries` | [07 / main.rs](src/bin/07-unsafe-and-ffi/main.rs) |

## 如何判断示例说明了什么

- 编译期反例保留在相邻注释中，标注拒绝原因；取消注释可单独观察。故意 panic、死锁和 UB 不在正常运行路径执行。
- 单元测试与运行期断言检查输入、状态和结果：端口缺失/非法/边界值、计数器耗尽、RefCell 冲突、Weak 失效、重复索引/越界/空切片/ZST、Cow 的 borrowed/owned 分支等。
- 并发丢更新用 Barrier 固定交错；异步取消用满队列与 `select! { biased; ... }` 固定选择。任务完成通过 JoinHandle/oneshot 确认，不依靠 sleep 猜测调度。
- 05 的[并发十点速查](../blocksmith-codex/backend-engineering/rust-language/05-concurrency-and-synchronization.md#并发十点速查)逐项对应函数与拒绝示例。`published_payload` 在 join 前断言发布值，`seqcst_observations` 只断言双方不能都读到初始值；普通运行测试不穷尽弱内存行为、不证明所有调度下的活性，P2 的无锁算法与模型检查仍留在笔记中。
- 06 使用单线程 Tokio runtime 展示任务并发；`spawn_blocking` 将阻塞工作交给阻塞线程池。示例没有真实网络 I/O，也不测多线程 runtime 性能。
- 07 的 [FFI 模块](src/bin/07-unsafe-and-ffi/ffi.rs) 在 Rust 内调用 `extern "C"` 函数，演示句柄协议；没有构建供 C 调用的库，不能据此证明跨语言 ABI、异常边界或 allocator 集成正确。
- `cargo check` 不执行函数体；行为测试不证明所有 unsafe 路径 sound，也不证明性能。Miri、跨语言集成、MSRV 和 benchmark 需要另行验证。

笔记链接假定本仓库与 `blocksmith-codex` 位于同一父目录；单独克隆时示例仍可运行。

## API 依据

- [std::thread::spawn 的 Send 与生命周期约束](https://doc.rust-lang.org/std/thread/fn.spawn.html)
- [std::sync::atomic::Ordering](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html)
- [Tokio 1.53.1 features 与 runtime](https://docs.rs/tokio/1.53.1/tokio/)
- [Tokio select 的取消安全约定](https://docs.rs/tokio/1.53.1/tokio/macro.select.html#cancellation-safety)
- [Box::from_raw 的安全前提](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw)
