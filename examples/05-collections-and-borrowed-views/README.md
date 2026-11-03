# 05 集合与借用视图：按示例学习

先运行基础动作，再看边界和组合应用。表格顺序就是章节入口顺序；P0/P1 表示复习优先级，具体示例仍按前置关系阅读。运行前预测文件头部的状态与结果，再按“改一处”观察变化。

[配套笔记](../../../blocksmith-codex/computer-science/04-language/rust/05-collections-and-borrowed-views.md) · [仓库索引](../../README.md)

| 优先级 | 笔记小节 | 示例文件 | 运行命令 | 观察重点 |
| --- | --- | --- | --- | --- |
| P1 | [先借一段文本](../../../blocksmith-codex/computer-science/04-language/rust/05-collections-and-borrowed-views.md#先借一段文本) | [string-view.rs](string-view.rs) | `cargo run --locked --example 05-string-view` | 基础：String 拥有文本，&str 只借用，原容器仍可用。 |
| P1 | [再借一段序列](../../../blocksmith-codex/computer-science/04-language/rust/05-collections-and-borrowed-views.md#再借一段序列) | [slice-view.rs](slice-view.rs) | `cargo run --locked --example 05-slice-view` | 基础：slice 只观察子范围，原 Vec 长度不变。 |
| P1 | [通过 slice 修改已有元素](../../../blocksmith-codex/computer-science/04-language/rust/05-collections-and-borrowed-views.md#通过slice修改已有元素) | [slice-mutation.rs](slice-mutation.rs) | `cargo run --locked --example 05-slice-mutation` | 基础：20 改为 25，只修改已有元素，长度仍为 3。 |
| P1 | [借用结束后再增长](../../../blocksmith-codex/computer-science/04-language/rust/05-collections-and-borrowed-views.md#借用结束后再增长) | [vec-growth.rs](vec-growth.rs) | `cargo run --locked --example 05-vec-growth` | 基础：通过 Vec push，长度从 3 变为 4。 |
| P1 | [拥有固定长度的 Box 切片](../../../blocksmith-codex/computer-science/04-language/rust/05-collections-and-borrowed-views.md#转为拥有固定长度的box切片) | [boxed-slice.rs](boxed-slice.rs) | `cargo run --locked --example 05-boxed-slice` | 基础：消费 Vec 后由 Box 切片拥有元素，再借出只读视图。 |
| P1 | [String 与 str](../../../blocksmith-codex/computer-science/04-language/rust/05-collections-and-borrowed-views.md#p1-string与str) | [strings-utf8.rs](strings-utf8.rs) | `cargo run --locked --example 05-strings-utf8` | 边界：字节长度、Unicode 标量数量与 UTF-8 切片边界。 |
| P1 | [Vec 与 slice](../../../blocksmith-codex/computer-science/04-language/rust/05-collections-and-borrowed-views.md#p1-vec与slice) | [vec-slice.rs](vec-slice.rs) | `cargo run --locked --example 05-vec-slice` | 边界对照：结束元素借用再 push；保留 push 后继续读取的拒编对照。 |
| P1 | [HashMap](../../../blocksmith-codex/computer-science/04-language/rust/05-collections-and-borrowed-views.md#p1-hashmap) | [hashmap-entry.rs](hashmap-entry.rs) | `cargo run --locked --example 05-hashmap-entry` | 接口应用：查询借用、插入与 entry 接管键；惰性默认值与稳定展示。 |
| P1 | [Cow](../../../blocksmith-codex/computer-science/04-language/rust/05-collections-and-borrowed-views.md#p1-cow) | [cow.rs](cow.rs) | `cargo run --locked --example 05-cow` | 接口应用：区分借用结果与拥有结果，首次修改触发拥有。 |

整章入口调用上表同一份代码：

```sh
cargo run --locked --bin 05-collections-and-borrowed-views
```

注释中的拒绝编译、故意 panic 或死锁对照不会在正常运行中执行，也不等于已被自动测试。一次运行和断言只支持覆盖路径内的结论。
