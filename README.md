# rust-tutorial

通过独立、可运行的 Rust 示例学习语言语义。十个教程主题与 [Rust 学习地图](../blocksmith-codex/computer-science/04-language/rust/rust-semantics-ownership-types-and-standard-library.md)、[重点回顾图](../blocksmith-codex/computer-science/04-language/rust/00-quick-review.md)及十篇正文使用相同编号和标识。

保留 Rust 2021 Edition；08 异步篇使用锁定的 Tokio 1.53.1（`macros`、`rt`、`sync`、`time`），其余语言示例和 09 最小库使用标准库。Tokio 是根 package 依赖，首次构建需要锁定依赖。示例属于泛化教学模型，P0/P1/P2 是复习顺序，不是能力或性能证明。

## 运行与阅读顺序

```sh
# 默认入口与 00 入门保留
cargo run --locked --bin rust-tutorial
cargo run --locked --bin 00-hello-world

# 按知识点运行：一个入口先观察一个动作
cargo run --locked --example 01-struct-basics
cargo run --locked --example 02-move-value
cargo run --locked --example 04-result-question-mark
cargo run --locked --example 06-weak-lifecycle
cargo run --locked --example 07-mutex-basics
cargo run --locked --example 08-lazy-future-await

# 综合回顾：先完成对应基础示例
cargo run --locked --example 02-move-copy-clone
cargo run --locked --example 07-shared-store

# 整章入口
cargo run --locked --bin 01-language-foundations-and-control-flow
cargo run --locked --bin 08-async-future-and-tokio

# 格式、编译与行为分别验证
cargo fmt --check
cargo check --locked --all-targets
cargo test --locked --all-targets

# 09 独立库：根包检查不代替单元、集成和文档测试
cargo fmt --manifest-path examples/09-modules-cargo-macros-and-testing/limit-demo/Cargo.toml --check
cargo test --locked --offline --manifest-path examples/09-modules-cargo-macros-and-testing/limit-demo/Cargo.toml --target-dir target/09-limit-demo

cargo tree --locked -e features
```

各章索引与聚合入口采用相同顺序：基础动作 → 机制与边界 → 综合应用。首轮先运行基础短例，观察一项输入怎样改变状态；理解后再运行综合例。原综合命令保留，README 明确标注其位置。P0/P1 仍是笔记复习分级，可在各阶段内选读；最后按岗位深入笔记中的 P2。集合、闭包、宏已回到各自主题，没有独立“补充篇”。本仓库不实现生产执行器、无锁算法或复杂 pin projection。

## 十个主题与对应正文

| 主题与章节入口 | 主要内容 | 对应正文 |
| --- | --- | --- |
| [01-language-foundations-and-control-flow](examples/01-language-foundations-and-control-flow/README.md) | 绑定、类型、表达式、分支、循环、模式组合与所有权 | [语言基础与控制流](../blocksmith-codex/computer-science/04-language/rust/01-language-foundations-and-control-flow.md) |
| [02-ownership-and-lifetimes](examples/02-ownership-and-lifetimes/README.md) | Move/Copy/Clone、Borrow/NLL、Lifetime、partial move、reborrow | [所有权、借用与生命周期](../blocksmith-codex/computer-science/04-language/rust/02-ownership-and-lifetimes.md) |
| [03-functions-generics-and-traits](examples/03-functions-generics-and-traits/README.md) | Trait Bound、关联类型、impl/dyn、newtype、闭包与 Iterator | [函数、泛型与 Trait](../blocksmith-codex/computer-science/04-language/rust/03-functions-generics-and-traits.md) |
| [04-error-handling](examples/04-error-handling/README.md) | Option/Result/?、错误分类与 source、panic 和恢复边界 | [错误处理](../blocksmith-codex/computer-science/04-language/rust/04-error-handling.md) |
| [05-collections-and-borrowed-views](examples/05-collections-and-borrowed-views/README.md) | String/str、Vec/slice、HashMap/BTreeMap、Cow 与容量 | [集合与借用视图](../blocksmith-codex/computer-science/04-language/rust/05-collections-and-borrowed-views.md) |
| [06-smart-pointers-and-resource-management](examples/06-smart-pointers-and-resource-management/README.md) | Box/Rc/Arc/Weak、Cell/RefCell、Deref、Drop/RAII 与 DST | [智能指针与资源管理](../blocksmith-codex/computer-science/04-language/rust/06-smart-pointers-and-resource-management.md) |
| [07-concurrency-and-synchronization](examples/07-concurrency-and-synchronization/README.md) | 线程、Send/Sync、锁、通道、逻辑竞态与原子内存序 | [并发与同步](../blocksmith-codex/computer-science/04-language/rust/07-concurrency-and-synchronization.md) |
| [08-async-future-and-tokio](examples/08-async-future-and-tokio/README.md) | async/await → join/spawn → 消息收发/select → 阻塞卸载；按需深入类型、取消与 poll/Pin | [Async、Future 与 Tokio](../blocksmith-codex/computer-science/04-language/rust/08-async-future-and-tokio.md) |
| [09-modules-cargo-macros-and-testing](examples/09-modules-cargo-macros-and-testing/README.md) | 模块可见性、公开重导出、宏求值及单元/集成/文档测试 | [模块、Cargo、宏与测试](../blocksmith-codex/computer-science/04-language/rust/09-modules-cargo-macros-and-testing.md) |
| [10-unsafe-ffi-and-layout](examples/10-unsafe-ffi-and-layout/README.md) | 裸指针、安全不变量、safe wrapper、FFI 句柄与表示边界 | [Unsafe、FFI 与表示边界](../blocksmith-codex/computer-science/04-language/rust/10-unsafe-ffi-and-layout.md) |

此前 45 个独立示例全部迁移，并新增 `01-control-flow-patterns`；第 08 主题再补充五个基础示例。本轮与正文同步新增 39 个基础入口，原 51 个命令全部保留，当前共 **90 个 Cargo example**。每个源码位于主题目录、提供 `pub fn main()`（Tokio 示例可用 `#[tokio::main]` 修饰 `pub async fn main()`），通过显式 `[[example]]` 注册为 `<章号>-<concept>`；每个 `test = true`。章节 binary 用 `#[path]` 导入同一份源码并依次调用，部分单元测试因此在 example 和 bin 中各执行一次，不能当作两套独立证据。

09 还包含独立 `limit-demo` library package：私有实现通过 `pub use` 公开重导出 `parse_limit`，单元测试检查内部处理，集成测试从外部 crate 调用 API，doctest 检查文档用法。它使用自己的 Cargo.lock，不加入 workspace；单独的普通 `cargo test` 才覆盖该工程的 2 个单元、1 个集成和 1 个文档测试。09 章节入口同源调用解析实现并运行宏，不能替代这些工程验证。

各示例开头说明问题、输入与状态变化、结果证据、改一处的观察点、边界和运行命令；章节索引列出优先级及配套笔记。注释中的拒编、panic 和死锁对照不在正常路径执行，也不由注释存在本身建立验证结论。

## 旧章节迁移

本次以十主题入口替换原 01–08，旧 bin/example 命令不再提供别名。章内示例 target 使用新章号，概念名不变；例如 `01-move-copy-clone` 改为 `02-move-copy-clone`。

| 原章节入口（已替换） | 新主题与知识点索引 |
| --- | --- |
| `01-ownership-and-borrowing` | [02 所有权、借用与生命周期](examples/02-ownership-and-lifetimes/README.md) |
| `02-type-system-and-error-handling` | [01 语言基础与控制流](examples/01-language-foundations-and-control-flow/README.md)、[04 错误处理](examples/04-error-handling/README.md) |
| `03-traits-generics-and-dispatch` | [03 函数、泛型与 Trait](examples/03-functions-generics-and-traits/README.md) |
| `04-memory-and-smart-pointers` | [06 智能指针与资源管理](examples/06-smart-pointers-and-resource-management/README.md) |
| `05-concurrency-and-synchronization` | [07 并发与同步](examples/07-concurrency-and-synchronization/README.md) |
| `06-async-future-and-tokio` | [08 Async、Future 与 Tokio](examples/08-async-future-and-tokio/README.md) |
| `07-unsafe-and-ffi` | [10 Unsafe、FFI 与表示边界](examples/10-unsafe-ffi-and-layout/README.md) |
| `08-collections-closures-and-engineering` | [05 集合与借用视图](examples/05-collections-and-borrowed-views/README.md)、[03 函数、泛型与 Trait](examples/03-functions-generics-and-traits/README.md)、[09 模块、Cargo、宏与测试](examples/09-modules-cargo-macros-and-testing/README.md) |

更早的十章入口也已替换，历史选题按以下最终位置查找：

| 更早入口（已替换） | 最终位置 |
| --- | --- |
| `01-values-types-patterns` | [语言基础与控制流](examples/01-language-foundations-and-control-flow/README.md) |
| `02-ownership-move-copy-clone` | [所有权、借用与生命周期](examples/02-ownership-and-lifetimes/README.md) |
| `03-borrowing-lifetimes` | [所有权、借用与生命周期](examples/02-ownership-and-lifetimes/README.md) |
| `04-strings-slices-collections` | [集合与借用视图](examples/05-collections-and-borrowed-views/README.md) |
| `05-enums-option-result` | [错误处理](examples/04-error-handling/README.md) |
| `06-generics-traits` | [函数、泛型与 Trait](examples/03-functions-generics-and-traits/README.md) |
| `07-smart-pointers` | [智能指针与资源管理](examples/06-smart-pointers-and-resource-management/README.md) |
| `08-closures-iterators` | [函数、泛型与 Trait](examples/03-functions-generics-and-traits/README.md) |
| `09-send-threads-async` | [并发与同步](examples/07-concurrency-and-synchronization/README.md)、[Async、Future 与 Tokio](examples/08-async-future-and-tokio/README.md) |
| `10-unsafe-boundaries` | [Unsafe、FFI 与表示边界](examples/10-unsafe-ffi-and-layout/README.md) |

## 如何判断示例说明了什么

- 编译期反例保留在相邻注释中，标注拒绝原因；取消注释可单独观察。故意 panic、死锁和 UB 不在正常运行路径执行。
- 入门 main 优先展示目标动作；精确调用次数、析构日志等辅助验证放在测试。单元测试与运行期断言检查输入、状态和结果：端口缺失/非法/边界值、计数器耗尽、RefCell 冲突、Weak 失效、重复索引/越界/空切片/ZST、Cow 的 borrowed/owned 分支等。
- 并发丢更新用 Barrier 固定交错；异步取消用满队列与 `select! { biased; ... }` 固定选择。任务完成通过 JoinHandle/oneshot 确认，不依靠 sleep 猜测调度。
- 07 的[并发与同步](../blocksmith-codex/computer-science/04-language/rust/07-concurrency-and-synchronization.md#p1-原子性与内存序)逐项对应函数与拒绝示例。`published_payload` 在 join 前断言发布值，`seqcst_observations` 只断言双方不能都读到初始值；普通运行测试不穷尽弱内存行为、不证明所有调度下的活性，P2 的无锁算法与模型检查仍留在笔记中。
- 08 先逐例解释 async/await、join/spawn、mpsc、oneshot、select 与阻塞卸载，再深入任务退出、Send、poll/Pin 和取消。异步例子各自使用单线程 Tokio runtime 展示任务并发，整章同步入口会顺序创建多个相同配置的 runtime。各例子在返回前等待教学任务完成或确认取消，不跨例子共享任务；`spawn_blocking` 将阻塞工作交给阻塞线程池。示例没有真实网络 I/O，也不测多线程 runtime 性能。
- 10 的 [FFI 句柄示例](examples/10-unsafe-ffi-and-layout/ffi-handle.rs) 在 Rust 内调用 `extern "C"` 函数，演示句柄协议；没有构建供 C 调用的库，不能据此证明跨语言 ABI、异常边界或 allocator 集成正确。
- `cargo check` 不执行函数体；行为测试不证明所有 unsafe 路径 sound，也不证明性能。Miri、跨语言集成、MSRV 和 benchmark 需要另行验证。

笔记链接假定本仓库与 `blocksmith-codex` 位于同一父目录；单独克隆时示例仍可运行。

## 学习方式参考

- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)：以完整、可运行的小例子展示一个概念；本仓库借鉴它的学习粒度。
- [Rustlings](https://rustlings.rust-lang.org/usage/)：按主题组织小练习，通过编译和测试反馈学习；本仓库借鉴它的主题索引和改动观察方式。
- [Comprehensive Rust 中文版](https://google.github.io/comprehensive-rust/zh-CN/)：短示例配合解释与练习，覆盖并发与异步等专题；用于高级章节的延伸阅读。

十主题重组保留原有示例并补充最小控制流与工程验证代码；本轮按正文增加独立基础入口，将原综合例后置；以上项目提供学习方式参考，没有复制其示例或文档内容。

## API 依据

- [std::thread::spawn 的 Send 与生命周期约束](https://doc.rust-lang.org/std/thread/fn.spawn.html)
- [std::sync::atomic::Ordering](https://doc.rust-lang.org/std/sync/atomic/enum.Ordering.html)
- [Tokio 1.53.1 features 与 runtime](https://docs.rs/tokio/1.53.1/tokio/)
- [Tokio select 的取消安全约定](https://docs.rs/tokio/1.53.1/tokio/macro.select.html#cancellation-safety)
- [Box::from_raw 的安全前提](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw)
