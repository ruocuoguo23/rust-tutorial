# 08 容器、闭包与工程补充：按示例学习

先读文件开头的问题与输入，预测状态变化，再单独运行并对照输出或断言。最后按“改一处”提示修改代码，观察结果，复习时从表格中的问题回想机制。注释中的编译失败或 panic 对照不会在正常运行中执行。

本章均为 P1 补充内容；整章入口依次调用同一份示例代码：

```sh
cargo run --locked --bin 08-collections-closures-and-engineering
```

| 优先级 | 笔记小节 | 示例文件 | 运行命令 | 观察重点 |
| --- | --- | --- | --- | --- |
| P1 | [P1 String与str](../../../blocksmith-codex/backend-engineering/rust-language/08-collections-closures-and-engineering.md#p1-string与str) | [strings-utf8.rs](strings-utf8.rs) | `cargo run --locked --example 08-strings-utf8` | 字节长度、Unicode 标量数量与切片边界有什么不同？ |
| P1 | [P1 Vec与slice](../../../blocksmith-codex/backend-engineering/rust-language/08-collections-closures-and-engineering.md#p1-vec与slice) | [vec-slice.rs](vec-slice.rs) | `cargo run --locked --example 08-vec-slice` | 元素借用在何时结束，为什么 push 可能与借用冲突？ |
| P1 | [P1 HashMap](../../../blocksmith-codex/backend-engineering/rust-language/08-collections-closures-and-engineering.md#p1-hashmap) | [hashmap-entry.rs](hashmap-entry.rs) | `cargo run --locked --example 08-hashmap-entry` | 查询借用、插入与 entry 接管 String；默认值闭包仅缺失时执行；显式排序。 |
| P1 | [P1 Cow](../../../blocksmith-codex/backend-engineering/rust-language/08-collections-closures-and-engineering.md#p1-cow) | [cow.rs](cow.rs) | `cargo run --locked --example 08-cow` | 哪条路径借用输入，哪次修改开始拥有数据？ |
| P1 | [P1 Closure与Iterator](../../../blocksmith-codex/backend-engineering/rust-language/08-collections-closures-and-engineering.md#p1-closure与iterator) | [closure-traits.rs](closure-traits.rs) | `cargo run --locked --example 08-closure-traits` | 闭包如何使用捕获值，是否还能再次调用？ |
| P1 | [P1 Closure与Iterator](../../../blocksmith-codex/backend-engineering/rust-language/08-collections-closures-and-engineering.md#p1-closure与iterator) | [lazy-iterators.rs](lazy-iterators.rs) | `cargo run --locked --example 08-lazy-iterators` | 构造 map 还是 collect 执行闭包，何时转移元素所有权？ |
| P1 | [P1 Macro](../../../blocksmith-codex/backend-engineering/rust-language/08-collections-closures-and-engineering.md#p1-macro) | [macro-evaluation.rs](macro-evaluation.rs) | `cargo run --locked --example 08-macro-evaluation` | 表达式展开后求值几次，副作用是否被重复执行？ |

独立 example 是 Cargo 编译的 crate；整章入口通过模块导入同一份源码，module 与 crate 的边界因此不同。[笔记中的 Cargo 与 Feature 小节](../../../blocksmith-codex/backend-engineering/rust-language/08-collections-closures-and-engineering.md#p1-cratecargo与feature)和[仓库验证说明](../../README.md#如何判断示例说明了什么)继续解释工程命令与证据边界。
