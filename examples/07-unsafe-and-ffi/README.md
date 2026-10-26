# 07 unsafe 与 FFI：按示例学习

先读文件开头的问题与输入，预测状态变化，再单独运行并对照输出或断言。最后按“改一处”提示修改代码，观察结果，复习时从表格中的问题回想机制。注释中的编译失败或 panic 对照不会在正常运行中执行。

整章入口保持原命令，按 P0 → P1 顺序调用同一份示例代码：

```sh
cargo run --locked --bin 07-unsafe-and-ffi
```

| 优先级 | 笔记小节 | 示例文件 | 运行命令 | 观察重点 |
| --- | --- | --- | --- | --- |
| P0 | [P0 Safety Invariant与Safe Abstraction](../../../blocksmith-codex/backend-engineering/rust-language/07-unsafe-and-ffi.md#p0-safety-invariant与safe-abstraction) | [raw-pointer-safe-wrapper.rs](raw-pointer-safe-wrapper.rs) | `cargo run --locked --example 07-raw-pointer-safe-wrapper` | 安全函数怎样保证返回的两个可变引用有效且不重叠？ |
| P1 | [P1 FFI协议与释放责任](../../../blocksmith-codex/backend-engineering/rust-language/07-unsafe-and-ffi.md#p1-ffi协议与释放责任) | [ffi-handle.rs](ffi-handle.rs) | `cargo run --locked --example 07-ffi-handle` | 句柄何时分配、读取、释放，谁负责唯一一次释放？ |

原始指针示例保留完整 `SAFETY` 前提，并与 `split_at_mut` 的安全替代实现对照。FFI 示例只展示 Rust → Rust 的 C ABI 调用与所有权协议，不含真实 C 库链接，也不执行双重释放、悬垂访问或其他未定义行为。测试通过不能证明 unsafe 封装在所有情形下都健全。
