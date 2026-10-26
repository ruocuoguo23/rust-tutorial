# 04：存储、智能指针与析构

先看 owner 持有什么，再看共享、修改权限和资源释放。每个文件都可独立运行；同一个文件中的 P0 / P1 内容按顺序执行，整章入口从同源代码分阶段调用，保持 P0 → P1。

| 优先级 | 笔记小节 | 示例文件 | 运行命令 | 观察重点 |
| --- | --- | --- | --- | --- |
| P0 / P1 | [Stack、Heap 与 Box](../../../blocksmith-codex/backend-engineering/rust-language/04-memory-and-smart-pointers.md#p0-stackheap与box)、[Cell 与 DST](../../../blocksmith-codex/backend-engineering/rust-language/04-memory-and-smart-pointers.md#p1-cell与dst) | [box-dst.rs](box-dst.rs) | `cargo run --locked --example 04-box-dst` | Box move 转移 owner；Box slice 拥有内容，slice 借用携带长度。 |
| P0 | [Rc、Weak 与 Arc](../../../blocksmith-codex/backend-engineering/rust-language/04-memory-and-smart-pointers.md#p0-rcweak与arc) | [shared-ownership.rs](shared-ownership.rs) | `cargo run --locked --example 04-shared-ownership` | Arc clone 共享同一分配；Weak 不保持目标存活。 |
| P0 / P1 | [RefCell](../../../blocksmith-codex/backend-engineering/rust-language/04-memory-and-smart-pointers.md#p0-refcell)、[Cell 与 DST](../../../blocksmith-codex/backend-engineering/rust-language/04-memory-and-smart-pointers.md#p1-cell与dst) | [interior-mutability.rs](interior-mutability.rs) | `cargo run --locked --example 04-interior-mutability` | &self 记录器追加日志；读 guard 存活时写借用失败；Cell 可以整体替换非 Copy 的 String。 |
| P0 | [Drop 与 RAII](../../../blocksmith-codex/backend-engineering/rust-language/04-memory-and-smart-pointers.md#p0-drop与raii) | [drop-raii.rs](drop-raii.rs) | `cargo run --locked --example 04-drop-raii` | 显式 drop 先执行，作用域清理按逆声明顺序执行。 |
| P0 | [Deref](../../../blocksmith-codex/backend-engineering/rust-language/04-memory-and-smart-pointers.md#p0-deref) | [deref-coercion.rs](deref-coercion.rs) | `cargo run --locked --example 04-deref-coercion` | 自定义 Name 借作 &str，没有转移 String 所有权。 |

整章运行：

```sh
cargo run --locked --bin 04-memory-and-smart-pointers
```

每个文件开头包含输入、状态变化、结果证据和“改一处”的观察点。修改后先读编译器反馈，再核对输出与断言；注释中故意失败的代码保持注释。
