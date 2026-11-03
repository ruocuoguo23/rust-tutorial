# 10 Unsafe、FFI 与表示边界：按示例学习

先运行基础动作，再看边界和组合应用。表格顺序就是章节入口顺序；P0/P1 表示复习优先级，具体示例仍按前置关系阅读。运行前预测文件头部的状态与结果，再按“改一处”观察变化。

[配套笔记](../../../blocksmith-codex/computer-science/04-language/rust/10-unsafe-ffi-and-layout.md) · [仓库索引](../../README.md)

| 优先级 | 笔记小节 | 示例文件 | 运行命令 | 观察重点 |
| --- | --- | --- | --- | --- |
| P0 | [Raw Pointer](../../../blocksmith-codex/computer-science/04-language/rust/10-unsafe-ffi-and-layout.md#p0-raw-pointer) | [raw-pointer-access.rs](raw-pointer-access.rs) | `cargo run --locked --example 10-raw-pointer-access` | 基础：创建裸指针，再在有效性证明下将 7 改为 8。 |
| P0 | [unsafe fn 与 unsafe block](../../../blocksmith-codex/computer-science/04-language/rust/10-unsafe-ffi-and-layout.md#p0-unsafe-fn与unsafe-block) | [unsafe-call.rs](unsafe-call.rs) | `cargo run --locked --example 10-unsafe-call` | 责任：调用者保证读取条件，实现者用局部 unsafe block 执行操作。 |
| P0 | [Safety Invariant 与 Safe Abstraction](../../../blocksmith-codex/computer-science/04-language/rust/10-unsafe-ffi-and-layout.md#p0-safety-invariant与safe-abstraction) | [raw-pointer-safe-wrapper.rs](raw-pointer-safe-wrapper.rs) | `cargo run --locked --example 10-raw-pointer-safe-wrapper` | 封装与边界：校验不同且在界内的下标，返回不重叠的可变引用，与 safe 写法对照。 |
| P1 | [FFI 协议与释放责任](../../../blocksmith-codex/computer-science/04-language/rust/10-unsafe-ffi-and-layout.md#p1-ffi协议与释放责任) | [ffi-handle.rs](ffi-handle.rs) | `cargo run --locked --example 10-ffi-handle` | 协议应用：创建、通过入口读取、唯一释放；精确析构计数留测试。 |

整章入口调用上表同一份代码：

```sh
cargo run --locked --bin 10-unsafe-ffi-and-layout
```

注释中的拒绝编译、故意 panic 或死锁对照不会在正常运行中执行，也不等于已被自动测试。一次运行和断言只支持覆盖路径内的结论。

FFI 句柄示例在 Rust 内调用 extern "C" 函数；未构建供 C 调用的库，不证明真实跨语言 ABI、allocator 或异常边界集成。null 释放是本协议约定的合法空操作，不能据此验证任意外部指针。
