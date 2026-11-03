# 04 错误处理：按示例学习

先分别处理 Option 与 Result，再比较显式返回和两种 ?，最后组合 Result<Option>。每个新入口可以独立运行；旧配置策略与错误分类入口继续保留在后面。

先预测文件开头描述的输入、状态变化和结果，再运行示例，最后按“改一处”观察变化。下表与章节入口保持相同的基础 → 机制 → 组合顺序，P0/P1 表示章内优先级。

[配套笔记](../../../blocksmith-codex/computer-science/04-language/rust/04-error-handling.md) · [仓库索引](../../README.md)

| 优先级 | 笔记小节 | 示例文件 | 运行命令 | 观察重点 |
| --- | --- | --- | --- | --- |
| P0 | [Option 有值与缺失](../../../blocksmith-codex/computer-science/04-language/rust/04-error-handling.md#option分别处理有值与缺失) | [option-basics.rs](option-basics.rs) | `cargo run --locked --example 04-option-basics` | Some 采用原值，None 的默认值由谁选择？ |
| P0 | [Result 成功与失败](../../../blocksmith-codex/computer-science/04-language/rust/04-error-handling.md#result分别处理成功与失败) | [result-basics.rs](result-basics.rs) | `cargo run --locked --example 04-result-basics` | 有效文本、非法数字、越界分别进入哪个 match 分支？ |
| P0 | [显式错误返回](../../../blocksmith-codex/computer-science/04-language/rust/04-error-handling.md#显式返回把失败交还调用方) | [result-explicit-return.rs](result-explicit-return.rs) | `cargo run --locked --example 04-result-explicit-return` | Err 怎样提前返回，跳过后面的成功路径？ |
| P0 | [Result 的问号](../../../blocksmith-codex/computer-science/04-language/rust/04-error-handling.md#result的问号缩短同一条传播路径) | [result-question-mark.rs](result-question-mark.rs) | `cargo run --locked --example 04-result-question-mark` | 同样的输入，? 怎样缩短上一例的错误传播代码？ |
| P0 | [Option 的问号](../../../blocksmith-codex/computer-science/04-language/rust/04-error-handling.md#option的问号缺失时提前返回none) | [option-question-mark.rs](option-question-mark.rs) | `cargo run --locked --example 04-option-question-mark` | None 从哪里提前返回？空文本没有单词又是哪一步的结果？ |
| P0 | [Result 内 Option 三态](../../../blocksmith-codex/computer-science/04-language/rust/04-error-handling.md#result内的option表达三个结果) | [result-option-states.rs](result-option-states.rs) | `cargo run --locked --example 04-result-option-states` | 缺失、有效配置、无效配置如何分别保留？ |
| P0 | [配置与传播组合](../../../blocksmith-codex/computer-science/04-language/rust/04-error-handling.md#p0-optionresult与问号) | [option-result-propagation.rs](option-result-propagation.rs) | `cargo run --locked --example 04-option-result-propagation` | 综合：缺失配置使用默认值，解析失败与缺失单词分别传播。 |
| P1 | [错误设计与 panic](../../../blocksmith-codex/computer-science/04-language/rust/04-error-handling.md#p1-错误设计与panic) | [typed-errors.rs](typed-errors.rs) | `cargo run --locked --example 04-typed-errors` | 解析错误与业务错误如何区分？source 保留什么？ |

整章入口调用上表同一份源码：

```sh
cargo run --locked --bin 04-error-handling
```

注释中的拒绝编译、故意 panic 或死锁对照不会在正常运行中执行，也不等于已被自动测试。一次运行和断言只支持覆盖路径内的结论。

`result-option-states` 保留 `Ok(None)`，不选择默认值；旧 `option-result-propagation` 主动把未配置转换成默认端口 8080，两种策略用途不同。新增辅助函数的测试覆盖正常、缺失、空白、非法文本、负数与 u16 边界；显式返回与 Result ? 使用相同输入对照。原综合入口的三个测试和 typed-errors 的错误分类/source 测试保持原样。
