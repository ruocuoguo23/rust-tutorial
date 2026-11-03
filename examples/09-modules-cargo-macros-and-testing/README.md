# 09 模块、Cargo、宏与测试：按示例学习

先选一个问题，预测文件开头描述的输入、状态变化和结果，再运行示例。按“改一处”观察编译器反馈或状态变化，最后回到笔记理解机制。P0/P1 表示章内学习顺序。

[配套笔记](../../../blocksmith-codex/computer-science/04-language/rust/09-modules-cargo-macros-and-testing.md) · [仓库索引](../../README.md)

按正文依次阅读：先看库的公开 API 与内部模块，再看三类测试，最后观察宏的求值。

整章入口按 P0 → P1 调用同一份代码：

```sh
cargo run --locked --bin 09-modules-cargo-macros-and-testing
```

注释中的拒绝编译、故意 panic 或死锁对照不会在正常运行中执行，也不等于已被自动测试。一次运行和断言只支持覆盖路径内的结论。

## 独立最小库：模块与三种测试

[limit-demo](limit-demo/Cargo.toml) 是 Rust 2021 Edition 的无外部依赖库。它复用笔记中的 `parse_limit`：私有 `config` 模块实现解析，`lib.rs` 公开重导出函数，隐藏实现路径。

- [库入口](limit-demo/src/lib.rs)：先看 `pub use` 提供的公开路径，内部模块保持私有。
- [内部实现与单元测试](limit-demo/src/config.rs)：先只读 `parse_limit` 和私有 `cleaned`，再看下面的单元测试；解析和错误传播已在第 04 篇学过。
- [集成测试](limit-demo/tests/public_api.rs)：作为外部 crate 只调用公开 API，检查 0 与 65535。
- 文档注释中的 doctest：从 `limit_demo::parse_limit` 解析 `" 42 "`。

在教程仓库根运行：

```sh
cargo fmt --manifest-path examples/09-modules-cargo-macros-and-testing/limit-demo/Cargo.toml --check
cargo test --locked --offline --manifest-path examples/09-modules-cargo-macros-and-testing/limit-demo/Cargo.toml --target-dir target/09-limit-demo
```

这条普通 `cargo test` 单独运行 2 个单元测试、1 个集成测试和 1 个文档测试；根包的 `--all-targets` 不代替它。fixture 的独立 lockfile 已保留，不加入 workspace，也不增加根包依赖。

09 章节入口通过 `#[path]` 调用同源 `config.rs`，断言解析得到 42，并运行宏示例。这个模块调用不能证明跨 crate 可见性和文档测试；两者由独立库测试验证。config 的单测会在章节入口和库测试中各运行一次，不算两套独立证据。

## 最后观察宏的单次求值

| 优先级 | 笔记小节 | 示例文件 | 运行命令 | 观察重点 |
| --- | --- | --- | --- | --- |
| P1 | [P1 Macro](../../../blocksmith-codex/computer-science/04-language/rust/09-modules-cargo-macros-and-testing.md#p1-macro) | [macro-evaluation.rs](macro-evaluation.rs) | `cargo run --locked --example 09-macro-evaluation` | 表达式展开后求值几次，副作用是否被重复执行？ |

先运行看到 doubled=6、calls=1，再按源码注释把展开体改为 `($expression) + ($expression)`，观察 calls=2。重复求值的对照已加入测试；括号保证代码块作为加法操作数。展开结果仍需类型检查，不能据此认为任意类型都支持相加。
