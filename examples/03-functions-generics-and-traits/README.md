# 03 函数、泛型与 Trait：按示例学习

先运行一个独立入口，观察输入、状态变化和结果，再按“改一处”核对边界。表格与章节入口采用相同顺序：基础接口 → 闭包与迭代 → 机制补充 → 综合对照。P0/P1 表示章内学习顺序。

[配套笔记](../../../blocksmith-codex/computer-science/04-language/rust/03-functions-generics-and-traits.md) · [仓库索引](../../README.md)

通知相关的基础入口沿用正文的 Notifier/Email/Sms 场景；send 只生成字符串，不实际发送网络请求。最后的 trait-dispatch 保留原 Encoder/u32/bool 场景，用另一组实现回看相同的泛型、动态接口和返回类型机制。原有综合命令继续可用。

| 优先级 | 笔记小节 | 示例文件 | 运行命令 | 观察重点 |
| --- | --- | --- | --- | --- |
| P0 | [泛型：一种逻辑用于不同类型](../../../blocksmith-codex/computer-science/04-language/rust/03-functions-generics-and-traits.md#p0-generic与trait-bound) | [generic-basics.rs](generic-basics.rs) | `cargo run --locked --example 03-generic-basics` | u32 与 String 各选择一次 T；泛型不绕过所有权。 |
| P0 | [Trait 与能力约束](../../../blocksmith-codex/computer-science/04-language/rust/03-functions-generics-and-traits.md#p0-generic与trait-bound) | [trait-bound.rs](trait-bound.rs) | `cargo run --locked --example 03-trait-bound` | 同一 notify 借用两种实现；N: Notifier 使 send 可用。 |
| P0 | [参数位置 impl Trait](../../../blocksmith-codex/computer-science/04-language/rust/03-functions-generics-and-traits.md#p0-impl-traitdyn-trait与分发) | [impl-trait-argument.rs](impl-trait-argument.rs) | `cargo run --locked --example 03-impl-trait-argument` | 调用方选择满足约束的类型；与命名泛型对照。 |
| P0 | [返回位置 impl Trait](../../../blocksmith-codex/computer-science/04-language/rust/03-functions-generics-and-traits.md#p0-impl-traitdyn-trait与分发) | [impl-trait-return.rs](impl-trait-return.rs) | `cargo run --locked --example 03-impl-trait-return` | 实现方固定返回 Email，调用方依靠通知能力使用。 |
| P0 | [借用 dyn Trait](../../../blocksmith-codex/computer-science/04-language/rust/03-functions-generics-and-traits.md#p0-impl-traitdyn-trait与分发) | [dyn-borrowed.rs](dyn-borrowed.rs) | `cargo run --locked --example 03-dyn-borrowed` | &dyn Notifier 调用两种实现，本例不使用 Box。 |
| P0 | [拥有不同实现的集合](../../../blocksmith-codex/computer-science/04-language/rust/03-functions-generics-and-traits.md#p0-impl-traitdyn-trait与分发) | [dyn-collection.rs](dyn-collection.rs) | `cargo run --locked --example 03-dyn-collection` | Box 拥有值、dyn 提供接口；Vec 的元素类型仍统一。 |
| P0 | [Associated Type](../../../blocksmith-codex/computer-science/04-language/rust/03-functions-generics-and-traits.md#p0-associated-type) | [associated-types.rs](associated-types.rs) | `cargo run --locked --example 03-associated-types` | Item 固定为 u8；正常、空区间、逆区间与耗尽不溢出。 |
| P1 | [闭包：只读取捕获值](../../../blocksmith-codex/computer-science/04-language/rust/03-functions-generics-and-traits.md#closure捕获与调用能力) | [closure-fn.rs](closure-fn.rs) | `cargo run --locked --example 03-closure-fn` | 两次得到 4，外部 text 仍可使用。 |
| P1 | [闭包：修改捕获状态](../../../blocksmith-codex/computer-science/04-language/rust/03-functions-generics-and-traits.md#closure捕获与调用能力) | [closure-fn-mut.rs](closure-fn-mut.rs) | `cargo run --locked --example 03-closure-fn-mut` | 同一计数从 0 推进到 1、2，闭包需要 mut。 |
| P1 | [闭包：移出捕获值](../../../blocksmith-codex/computer-science/04-language/rust/03-functions-generics-and-traits.md#closure捕获与调用能力) | [closure-fn-once.rs](closure-fn-once.rs) | `cargo run --locked --example 03-closure-fn-once` | String 从闭包交给调用方，不能第二次取出。 |
| P1 | [Iterator：惰性执行](../../../blocksmith-codex/computer-science/04-language/rust/03-functions-generics-and-traits.md#iterator惰性执行与元素所有权) | [lazy-iterators.rs](lazy-iterators.rs) | `cargo run --locked --example 03-lazy-iterators` | 先打印准备收集，再处理名字；Cell 精确计数只在测试。 |
| P1 | [Iterator：借用、修改与消费](../../../blocksmith-codex/computer-science/04-language/rust/03-functions-generics-and-traits.md#组合例闭包参与迭代) | [iterator-ownership.rs](iterator-ownership.rs) | `cargo run --locked --example 03-iterator-ownership` | 依次观察 iter、iter_mut、owned Vec 的 into_iter。 |
| P1 | [coherence、newtype 与 From/Into](../../../blocksmith-codex/computer-science/04-language/rust/03-functions-generics-and-traits.md#p1-coherence与newtype) | [newtype-conversions.rs](newtype-conversions.rs) | `cargo run --locked --example 03-newtype-conversions` | 本地类型身份、空文本与可失败转换。 |
| P1 | [derive 与自定义相等](../../../blocksmith-codex/computer-science/04-language/rust/03-functions-generics-and-traits.md#p0-generic与trait-bound) | [derive-equality.rs](derive-equality.rs) | `cargo run --locked --example 03-derive-equality` | Clone 与按 x 比较分别承担什么责任？ |
| P1 | [综合：Closure 调用能力对照](../../../blocksmith-codex/computer-science/04-language/rust/03-functions-generics-and-traits.md#closure捕获与调用能力) | [closure-traits.rs](closure-traits.rs) | `cargo run --locked --example 03-closure-traits` | move + Fn、FnMut、FnOnce 与泛型 call_once 放在一起回看。 |
| P1 | [综合：泛型、dyn 与 impl Iterator 对照](../../../blocksmith-codex/computer-science/04-language/rust/03-functions-generics-and-traits.md#p0-impl-traitdyn-trait与分发) | [trait-dispatch.rs](trait-dispatch.rs) | `cargo run --locked --example 03-trait-dispatch` | Encoder 场景综合对照静态/动态路径、异构集合与固定返回类型。 |

整章入口顺序调用同一份示例代码：

```sh
cargo run --locked --bin 03-functions-generics-and-traits
```

注释中的拒绝编译、故意 panic 或死锁对照不会在正常运行中执行，也不等于已被自动测试。一次运行和断言只支持覆盖路径内的结论。
