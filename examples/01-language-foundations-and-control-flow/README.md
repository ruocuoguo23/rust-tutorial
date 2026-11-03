# 01 语言基础与控制流：按示例学习

先观察 Struct/Enum 的构造，再分别看 match 与 guard，最后组合控制流、模式和业务状态。原有综合示例保留在基础例之后。

先预测文件开头描述的输入、状态变化和结果，再运行示例，最后按“改一处”观察变化。下表与章节入口保持相同的基础 → 机制 → 组合顺序，P0/P1 表示章内优先级。

[配套笔记](../../../blocksmith-codex/computer-science/04-language/rust/01-language-foundations-and-control-flow.md) · [仓库索引](../../README.md)

| 优先级 | 笔记小节 | 示例文件 | 运行命令 | 观察重点 |
| --- | --- | --- | --- | --- |
| P0 | [表达式与绑定](../../../blocksmith-codex/computer-science/04-language/rust/01-language-foundations-and-control-flow.md#p0-表达式与绑定) | [expressions-shadowing.rs](expressions-shadowing.rs) | `cargo run --locked --example 01-expressions-shadowing` | 赋值与遮蔽有什么区别？分号如何改变块的结果类型？ |
| P0 | [Struct 构造与字段](../../../blocksmith-codex/computer-science/04-language/rust/01-language-foundations-and-control-flow.md#先构造struct并读取字段) | [struct-basics.rs](struct-basics.rs) | `cargo run --locked --example 01-struct-basics` | 输入坐标怎样构成 Point？怎样按字段名读回它们？ |
| P0 | [Enum 变体构造](../../../blocksmith-codex/computer-science/04-language/rust/01-language-foundations-and-control-flow.md#先构造enum的变体) | [enum-basics.rs](enum-basics.rs) | `cargo run --locked --example 01-enum-basics` | Pending 与 Done 怎样分别构造为同一种类型的值？ |
| P0 | [match 与穷尽性](../../../blocksmith-codex/computer-science/04-language/rust/01-language-foundations-and-control-flow.md#p0-match与穷尽性) | [match-basics.rs](match-basics.rs) | `cargo run --locked --example 01-match-basics` | 0、1 与其他次数怎样分别得到结果？删掉兜底分支会怎样？ |
| P0 | [match guard](../../../blocksmith-codex/computer-science/04-language/rust/01-language-foundations-and-control-flow.md#match-guard与变量遮蔽) | [match-guard.rs](match-guard.rs) | `cargo run --locked --example 01-match-guard` | 同一个 Some(5)，guard 成败为什么进入不同分支？ |
| P0 | [绑定与所有权](../../../blocksmith-codex/computer-science/04-language/rust/01-language-foundations-and-control-flow.md#p0-绑定与所有权) | [pattern-ownership.rs](pattern-ownership.rs) | `cargo run --locked --example 01-pattern-ownership` | &value、_ 与 _owned 分别借用、忽略还是移动？ |
| P0 | [控制流与模式组合](../../../blocksmith-codex/computer-science/04-language/rust/01-language-foundations-and-control-flow.md#p0-解构与模式组合) | [control-flow-patterns.rs](control-flow-patterns.rs) | `cargo run --locked --example 01-control-flow-patterns` | 综合：guard、范围、slice、while let 和循环终止怎样共同验证？ |
| P0 | [请求状态组合](../../../blocksmith-codex/computer-science/04-language/rust/01-language-foundations-and-control-flow.md#组合让请求状态携带数据) | [structs-enums-patterns.rs](structs-enums-patterns.rs) | `cargo run --locked --example 01-structs-enums-patterns` | 综合：用户查找、let-else、角色分支与带数据状态怎样协作？ |

整章入口调用上表同一份源码：

```sh
cargo run --locked --bin 01-language-foundations-and-control-flow
```

注释中的拒绝编译、故意 panic 或死锁对照不会在正常运行中执行，也不等于已被自动测试。一次运行和断言只支持覆盖路径内的结论。

基础 match/guard 的测试分别覆盖完整分支与条件成败；`structs-enums-patterns` 原有测试继续执行 let-else 成功与提前返回及全部 State 分支；`pattern-ownership` 保留 if-let 的 Some/None 测试；`control-flow-patterns` 保留范围边界、空切片、空栈、零次循环和终点测试。
