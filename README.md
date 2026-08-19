# rust-tutorial

这个仓库通过独立、可运行的 binary 示例学习 Rust 语言语义。项目保持 Rust 2021 Edition，并优先只使用标准库。

## 运行方式

```bash
# 运行单个主题
cargo run --bin 2-1-ownership

# 检查和测试全部示例
cargo check --bins
cargo test --bins
```

## 学习路线

| 章节 | 主题 | 示例 | 重点观察 |
| --- | --- | --- | --- |
| 1 | 值、表达式与模式 | [`1-variables.rs`](src/bin/1-variables.rs)、[`1-2-expressions-and-patterns.rs`](src/bin/1-2-expressions-and-patterns.rs) | shadowing、block value、`let else`、match |
| 2 | Ownership、Move、Copy 与 Clone | [`2-1-ownership.rs`](src/bin/2-1-ownership.rs) | Copy、move、显式 Clone、owned/borrowed API |
| 3 | Borrow 与 Lifetime | [`2-2-references-and-borrowing.rs`](src/bin/2-2-references-and-borrowing.rs) | NLL、reborrow、返回引用关系 |
| 4 | String、Slice 与 Collections | [`3-1-string-and-slice.rs`](src/bin/3-1-string-and-slice.rs)、[`3-3-collections.rs`](src/bin/3-3-collections.rs) | UTF-8 边界、Vec 借用、HashMap entry |
| 5 | Enum、Option、Result 与 `?` | [`3-2-enum.rs`](src/bin/3-2-enum.rs)、[`3-4-option-result.rs`](src/bin/3-4-option-result.rs) | 互斥状态、缺失、可恢复错误 |
| 6 | Generics 与 Trait Dispatch | [`4-1-generics-and-traits.rs`](src/bin/4-1-generics-and-traits.rs) | generic static dispatch 与 `dyn Trait` |
| 7 | Smart Pointer 与内部可变性 | [`4-2-smart-pointers.rs`](src/bin/4-2-smart-pointers.rs) | `Rc<RefCell<_>>`、动态借用、`Weak` |
| 8 | Closure 与 Iterator | [`5-1-closures-and-iterators.rs`](src/bin/5-1-closures-and-iterators.rs) | capture、lazy adapter、元素 ownership |
| 9 | Send、Thread 与 Async | [`5-2-send-and-async.rs`](src/bin/5-2-send-and-async.rs) | `Send` 编译期约束、跨 await 状态、`Arc` |
| 10 | Unsafe Boundary | [`6-1-unsafe-wrapper.rs`](src/bin/6-1-unsafe-wrapper.rs) | 验证边界与不重叠不变量后封装 unsafe |

补充示例：`attributes.rs` 与 `attributes_clone.rs` 展示 derive、手写 trait 实现和共享 Clone 语义。
