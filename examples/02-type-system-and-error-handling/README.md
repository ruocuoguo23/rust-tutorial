# 02 示例索引

先选一个问题，运行对应示例，再对照文件开头的输入、状态变化和结果。文件中的“改一处”提供一个具体观察点；修改后重新运行或检查编译器反馈，最后回到配套笔记理解机制。

[配套笔记](../../../blocksmith-codex/backend-engineering/rust-language/02-type-system-and-error-handling.md) · [仓库索引](../../README.md)

| 优先级 | 笔记小节 | 示例文件 | 运行命令 | 观察重点 |
| --- | --- | --- | --- | --- |
| P0 | [表达式与绑定](../../../blocksmith-codex/backend-engineering/rust-language/02-type-system-and-error-handling.md#表达式与绑定) | [expressions-shadowing.rs](expressions-shadowing.rs) | `cargo run --locked --example 02-expressions-shadowing` | 赋值与遮蔽有什么区别？分号如何改变块的结果类型？ |
| P0 | [Struct、Enum / Pattern Matching](../../../blocksmith-codex/backend-engineering/rust-language/02-type-system-and-error-handling.md#p0-struct与enum) | [structs-enums-patterns.rs](structs-enums-patterns.rs) | `cargo run --locked --example 02-structs-enums-patterns` | 状态如何携带各自的数据？let-else 的缺失分支如何退出？ |
| P0 | [Pattern Matching](../../../blocksmith-codex/backend-engineering/rust-language/02-type-system-and-error-handling.md#p0-pattern-matching) | [pattern-ownership.rs](pattern-ownership.rs) | `cargo run --locked --example 02-pattern-ownership` | &value、_ 与 _owned 分别借用、忽略还是移动？ |
| P0 | [Option、Result 与问号](../../../blocksmith-codex/backend-engineering/rust-language/02-type-system-and-error-handling.md#p0-optionresult与问号) | [option-result-propagation.rs](option-result-propagation.rs) | `cargo run --locked --example 02-option-result-propagation` | None、有效输入、非法输入怎样分别处理？? 从哪里提前返回？ |
| P1 | [错误设计与 panic](../../../blocksmith-codex/backend-engineering/rust-language/02-type-system-and-error-handling.md#p1-错误设计与panic) | [typed-errors.rs](typed-errors.rs) | `cargo run --locked --example 02-typed-errors` | 解析错误与业务错误如何区分？source 保留什么？ |

整章按 P0 → P1 顺序运行：

```sh
cargo run --locked --bin 02-type-system-and-error-handling
```

示例与整章入口调用同一份代码。注释中的无法编译或故意 panic 的对照代码不会在正常运行中执行；这类注释也不等于已通过自动测试。
