# 06 智能指针与资源管理：按示例学习

先运行基础动作，再看边界和组合应用。表格顺序就是章节入口顺序；P0/P1 表示复习优先级，具体示例仍按前置关系阅读。运行前预测文件头部的状态与结果，再按“改一处”观察变化。

[配套笔记](../../../blocksmith-codex/computer-science/04-language/rust/06-smart-pointers-and-resource-management.md) · [仓库索引](../../README.md)

| 优先级 | 笔记小节 | 示例文件 | 运行命令 | 观察重点 |
| --- | --- | --- | --- | --- |
| P0 | [Stack、Heap 与 Box](../../../blocksmith-codex/computer-science/04-language/rust/06-smart-pointers-and-resource-management.md#p0-stackheap与box) | [box-basics.rs](box-basics.rs) | `cargo run --locked --example 06-box-basics` | 基础：移动 Box 的 owner，目标值仍为 7。 |
| P0 | [Rc 共享同一个值](../../../blocksmith-codex/computer-science/04-language/rust/06-smart-pointers-and-resource-management.md#rc的clone共享同一个值) | [rc-sharing.rs](rc-sharing.rs) | `cargo run --locked --example 06-rc-sharing` | 基础：clone 共享 String，ptr_eq 与强计数说明共享存活。 |
| P0 | [Weak 不保活](../../../blocksmith-codex/computer-science/04-language/rust/06-smart-pointers-and-resource-management.md#weak观察目标但不保持它存活) | [weak-lifecycle.rs](weak-lifecycle.rs) | `cargo run --locked --example 06-weak-lifecycle` | 基础：最后一个强 owner 释放前后，upgrade 从 Some 变 None。 |
| P0 | [Arc 共享存活责任](../../../blocksmith-codex/computer-science/04-language/rust/06-smart-pointers-and-resource-management.md#arc共享存活责任) | [arc-sharing.rs](arc-sharing.rs) | `cargo run --locked --example 06-arc-sharing` | 基础：原子计数保护存活关系，不自动保护内部修改。 |
| P1 | [Cell](../../../blocksmith-codex/computer-science/04-language/rust/06-smart-pointers-and-resource-management.md#p1-cell) | [cell-counter.rs](cell-counter.rs) | `cargo run --locked --example 06-cell-counter` | 基础：共享引用通过 get/set 把计数从 0 改为 1。 |
| P0 | [RefCell](../../../blocksmith-codex/computer-science/04-language/rust/06-smart-pointers-and-resource-management.md#p0-refcell) | [refcell-borrow.rs](refcell-borrow.rs) | `cargo run --locked --example 06-refcell-borrow` | 基础：共享引用借出 RefMut，7 改为 8，作用域结束释放守卫。 |
| P0 | [Drop 与 RAII](../../../blocksmith-codex/computer-science/04-language/rust/06-smart-pointers-and-resource-management.md#p0-drop与raii) | [drop-raii.rs](drop-raii.rs) | `cargo run --locked --example 06-drop-raii` | 基础：先打印显式 drop 与作用域析构时点；精确日志留在测试。 |
| P0 | [Deref](../../../blocksmith-codex/computer-science/04-language/rust/06-smart-pointers-and-resource-management.md#p0-deref) | [deref-coercion.rs](deref-coercion.rs) | `cargo run --locked --example 06-deref-coercion` | 基础：自定义 Name 借作 &str，不转移内部 String 的所有权。 |
| P0 / P1 | [Box 与 DST](../../../blocksmith-codex/computer-science/04-language/rust/06-smart-pointers-and-resource-management.md#p1-dst) | [box-dst.rs](box-dst.rs) | `cargo run --locked --example 06-box-dst` | 表示边界：拥有数组与动态长度 Box 切片，区分 owner 与视图。 |
| P0 / P1 | [Cell / RefCell 应用](../../../blocksmith-codex/computer-science/04-language/rust/06-smart-pointers-and-resource-management.md#p0-内部可变性cell与refcell) | [interior-mutability.rs](interior-mutability.rs) | `cargo run --locked --example 06-interior-mutability` | 组合与边界：&self 记录器、守卫冲突、Cell 替换非 Copy 的 String。 |
| P0 | [父子关系与借用冲突](../../../blocksmith-codex/computer-science/04-language/rust/06-smart-pointers-and-resource-management.md#综合例父子关系与借用冲突) | [shared-ownership.rs](shared-ownership.rs) | `cargo run --locked --example 06-shared-ownership` | 组合应用：Rc 管存活、Weak 管反向关系、RefCell 管内部访问。 |

整章入口调用上表同一份代码：

```sh
cargo run --locked --bin 06-smart-pointers-and-resource-management
```

注释中的拒绝编译、故意 panic 或死锁对照不会在正常运行中执行，也不等于已被自动测试。一次运行和断言只支持覆盖路径内的结论。

原 `06-box-dst`、`06-interior-mutability` 与 `06-shared-ownership` 命令保留，分别在独立动作之后阅读。Rc/Arc 管理存活，Cell/RefCell 管理内部访问；跨线程同步仍需另看第 07 章。
