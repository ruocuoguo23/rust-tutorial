# rust-tutorial

这个仓库通过独立、可运行的 binary 示例学习 Rust 语言语义。项目保持 Rust 2021 Edition，并优先只使用标准库。

## 运行方式

```bash
# 运行单个主题
cargo run --bin 02-ownership-move-copy-clone

# 检查和测试全部示例
cargo check --bins
cargo test --bins
```

入门示例：[`00-hello-world`](src/bin/00-hello-world)。

## 学习路线

| 章节 | 主题 | 示例 | 重点观察 |
| --- | --- | --- | --- |
| 1 | 值、表达式与模式 | [`01-values-types-patterns`](src/bin/01-values-types-patterns) | mutation、shadowing、block value、`let else`、match |
| 2 | Ownership、Move、Copy 与 Clone | [`02-ownership-move-copy-clone`](src/bin/02-ownership-move-copy-clone) | Copy、move、显式 Clone、共享 Clone、owned/borrowed API |
| 3 | Borrow 与 Lifetime | [`03-borrowing-lifetimes`](src/bin/03-borrowing-lifetimes) | NLL、reborrow、返回引用关系 |
| 4 | String、Slice 与 Collections | [`04-strings-slices-collections`](src/bin/04-strings-slices-collections) | UTF-8 边界、Vec 借用、HashMap entry |
| 5 | Enum、Option、Result 与 `?` | [`05-enums-option-result`](src/bin/05-enums-option-result) | 互斥状态、缺失、可恢复错误 |
| 6 | Generics 与 Trait Dispatch | [`06-generics-traits`](src/bin/06-generics-traits) | generic/dyn dispatch、derive 与手写 trait 实现 |
| 7 | Smart Pointer 与内部可变性 | [`07-smart-pointers`](src/bin/07-smart-pointers) | `Rc<RefCell<_>>`、动态借用、`Weak` |
| 8 | Closure 与 Iterator | [`08-closures-iterators`](src/bin/08-closures-iterators) | capture、lazy adapter、元素 ownership |
| 9 | Send、Thread 与 Async | [`09-send-threads-async`](src/bin/09-send-threads-async) | `Send` 编译期约束、跨 await 状态、`Arc` |
| 10 | Unsafe Boundary | [`10-unsafe-boundaries`](src/bin/10-unsafe-boundaries) | 验证边界与不重叠不变量后封装 unsafe |
