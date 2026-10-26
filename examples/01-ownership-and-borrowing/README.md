# 01 示例索引

先选一个问题，运行对应示例，再对照文件开头的输入、状态变化和结果。文件中的“改一处”提供一个具体观察点；修改后重新运行或检查编译器反馈，最后回到配套笔记理解机制。

[配套笔记](../../../blocksmith-codex/backend-engineering/rust-language/01-ownership-and-borrowing.md) · [仓库索引](../../README.md)

| 优先级 | 笔记小节 | 示例文件 | 运行命令 | 观察重点 |
| --- | --- | --- | --- | --- |
| P0 | [Move、Copy、Clone](../../../blocksmith-codex/backend-engineering/rust-language/01-ownership-and-borrowing.md#p0-movecopyclone) | [move-copy-clone.rs](move-copy-clone.rs) | `cargo run --locked --example 01-move-copy-clone` | 赋值、按值传参后谁仍拥有值？Rc Clone 是否复制数据？ |
| P0 | [Borrow 与 NLL](../../../blocksmith-codex/backend-engineering/rust-language/01-ownership-and-borrowing.md#p0-borrow与nll) | [borrow-nll.rs](borrow-nll.rs) | `cargo run --locked --example 01-borrow-nll` | 共享引用最后使用之后，为何可以修改 String？ |
| P0 | [Lifetime](../../../blocksmith-codex/backend-engineering/rust-language/01-ownership-and-borrowing.md#p0-lifetime) | [lifetimes.rs](lifetimes.rs) | `cargo run --locked --example 01-lifetimes` | 返回引用来自哪一个输入？返回拥有值为何不会悬垂？ |
| P1 | [partial move 与取出字段](../../../blocksmith-codex/backend-engineering/rust-language/01-ownership-and-borrowing.md#p1-partial-move与取出字段) | [partial-move-take-replace.rs](partial-move-take-replace.rs) | `cargo run --locked --example 01-partial-move-take-replace` | 移出字段后什么仍可用？take/replace 留下什么值？ |
| P1 | [reborrow 与 static](../../../blocksmith-codex/backend-engineering/rust-language/01-ownership-and-borrowing.md#p1-reborrow与static) | [reborrow-static.rs](reborrow-static.rs) | `cargo run --locked --example 01-reborrow-static` | 短重借用何时结束？满足 static 的 String 为什么仍可 drop？ |

整章按 P0 → P1 顺序运行：

```sh
cargo run --locked --bin 01-ownership-and-borrowing
```

示例与整章入口调用同一份代码。注释中的无法编译或故意 panic 的对照代码不会在正常运行中执行；这类注释也不等于已通过自动测试。
