# 03 示例索引

先选一个问题，运行对应示例，再对照文件开头的输入、状态变化和结果。文件中的“改一处”提供一个具体观察点；修改后重新运行或检查编译器反馈，最后回到配套笔记理解机制。

[配套笔记](../../../blocksmith-codex/backend-engineering/rust-language/03-traits-generics-and-dispatch.md) · [仓库索引](../../README.md)

| 优先级 | 笔记小节 | 示例文件 | 运行命令 | 观察重点 |
| --- | --- | --- | --- | --- |
| P0 | [Generic / impl Trait、dyn Trait 与分发](../../../blocksmith-codex/backend-engineering/rust-language/03-traits-generics-and-dispatch.md#p0-impl-traitdyn-trait与分发) | [trait-dispatch.rs](trait-dispatch.rs) | `cargo run --locked --example 03-trait-dispatch` | 静态与动态分发如何得到相同结果？impl Trait 隐藏了什么类型？ |
| P0 | [Associated Type](../../../blocksmith-codex/backend-engineering/rust-language/03-traits-generics-and-dispatch.md#p0-associated-type) | [associated-types.rs](associated-types.rs) | `cargo run --locked --example 03-associated-types` | Item 如何固定为 u8？边界与耗尽状态为什么不溢出？ |
| P1 | [coherence 与 newtype / From、Into](../../../blocksmith-codex/backend-engineering/rust-language/03-traits-generics-and-dispatch.md#p1-coherence与newtype) | [newtype-conversions.rs](newtype-conversions.rs) | `cargo run --locked --example 03-newtype-conversions` | 本地类型身份如何允许实现 Display？失败转换如何表达？ |
| P1 | [Generic 与 Trait Bound（derive 补充）](../../../blocksmith-codex/backend-engineering/rust-language/03-traits-generics-and-dispatch.md#p0-generic与trait-bound) | [derive-equality.rs](derive-equality.rs) | `cargo run --locked --example 03-derive-equality` | 派生 Clone 与手写相等分别保证什么？为何 y 不同仍相等？ |

整章按 P0 → P1 顺序运行：

```sh
cargo run --locked --bin 03-traits-generics-and-dispatch
```

示例与整章入口调用同一份代码。注释中的无法编译或故意 panic 的对照代码不会在正常运行中执行；这类注释也不等于已通过自动测试。
