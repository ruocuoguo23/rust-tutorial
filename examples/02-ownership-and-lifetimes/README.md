# 02 所有权、借用与生命周期：按示例学习

先分别看 Move、Copy、Clone 的赋值状态，再看借用和生命周期。原有自定义类型、函数签名与共享 Clone 对照保留为后续组合；其中 Rc 的完整机制见[资源管理篇](../06-smart-pointers-and-resource-management/README.md)。

先预测文件开头描述的输入、状态变化和结果，再运行示例，最后按“改一处”观察变化。下表与章节入口保持相同的基础 → 机制 → 组合顺序，P0/P1 表示章内优先级。

[配套笔记](../../../blocksmith-codex/computer-science/04-language/rust/02-ownership-and-lifetimes.md) · [仓库索引](../../README.md)

| 优先级 | 笔记小节 | 示例文件 | 运行命令 | 观察重点 |
| --- | --- | --- | --- | --- |
| P0 | [Move 的直接状态](../../../blocksmith-codex/computer-science/04-language/rust/02-ownership-and-lifetimes.md#move原位置不再可用) | [move-value.rs](move-value.rs) | `cargo run --locked --example 02-move-value` | String 转交后，哪个位置还能读取文本？ |
| P0 | [Copy 的直接状态](../../../blocksmith-codex/computer-science/04-language/rust/02-ownership-and-lifetimes.md#copy赋值后两个位置都可用) | [copy-value.rs](copy-value.rs) | `cargo run --locked --example 02-copy-value` | i32 复制后，改变新位置为什么不影响原位置？ |
| P0 | [Clone 的直接状态](../../../blocksmith-codex/computer-science/04-language/rust/02-ownership-and-lifetimes.md#clone显式复制由实现决定复制什么) | [clone-value.rs](clone-value.rs) | `cargo run --locked --example 02-clone-value` | String 显式 clone 后，两份文本如何独立修改？ |
| P0 | [Borrow 与 NLL](../../../blocksmith-codex/computer-science/04-language/rust/02-ownership-and-lifetimes.md#p0-borrow与nll) | [borrow-nll.rs](borrow-nll.rs) | `cargo run --locked --example 02-borrow-nll` | 共享引用最后使用之后，为何可以修改 String？ |
| P0 | [Lifetime](../../../blocksmith-codex/computer-science/04-language/rust/02-ownership-and-lifetimes.md#p0-lifetime) | [lifetimes.rs](lifetimes.rs) | `cargo run --locked --example 02-lifetimes` | 返回引用来自哪一个输入？返回拥有值为何不会悬垂？ |
| P0 | [Copy/Clone 与 API 组合](../../../blocksmith-codex/computer-science/04-language/rust/02-ownership-and-lifetimes.md#字段都是-copy不代表结构体自动-copy) | [move-copy-clone.rs](move-copy-clone.rs) | `cargo run --locked --example 02-move-copy-clone` | 综合：自定义类型如何取得 Copy？借用、消费与 Rc clone 有何不同？ |
| P1 | [partial move 与取出字段](../../../blocksmith-codex/computer-science/04-language/rust/02-ownership-and-lifetimes.md#p1-partial-move与取出字段) | [partial-move-take-replace.rs](partial-move-take-replace.rs) | `cargo run --locked --example 02-partial-move-take-replace` | 移出字段后什么仍可用？take/replace 留下什么值？ |
| P1 | [reborrow 与 static](../../../blocksmith-codex/computer-science/04-language/rust/02-ownership-and-lifetimes.md#p1-reborrow与static) | [reborrow-static.rs](reborrow-static.rs) | `cargo run --locked --example 02-reborrow-static` | 短重借用何时结束？满足 static 的 String 为什么仍可 drop？ |

整章入口调用上表同一份源码：

```sh
cargo run --locked --bin 02-ownership-and-lifetimes
```

注释中的拒绝编译、故意 panic 或死锁对照不会在正常运行中执行，也不等于已被自动测试。一次运行和断言只支持覆盖路径内的结论。

三个基础状态例使用 main 内断言观察结果，不增加只复述赋值的单元测试。原有 derive、Rc、NLL、生命周期、partial move 和 reborrow 对照均保留；注释中的 Move 拒编路径需要另行核对，不能把成功运行当作拒编证据。
