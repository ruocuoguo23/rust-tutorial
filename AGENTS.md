# Repository Guidelines

## 项目目标

这个仓库通过一个个独立、可运行的 Rust 代码示例，帮助读者学习和验证 Rust 的语言特性。每个示例应聚焦一个明确主题，代码本身应当足以说明核心概念。

## 仓库结构

- `Cargo.toml`：项目配置。除非示例确实需要，优先只使用标准库。
- `src/main.rs`：默认入口，只保留最基础的运行示例。
- `examples/<chapter>-<topic>/<concept>.rs`：按知识点拆分的独立示例，通过 `Cargo.toml` 中的 `[[example]]` 注册。
- `examples/<chapter>-<topic>/README.md`：知识点、笔记小节、运行命令与观察重点的阅读索引。
- `src/bin/<chapter>-<topic>/main.rs`：章节聚合入口，导入并顺序调用同一份示例代码。

运行指定示例：

```bash
cargo run --locked --example 01-move-copy-clone
cargo run --locked --bin 01-ownership-and-borrowing
```

## 新增或修改示例

1. 一个目录只讲一个主要章节，避免把无关概念混在同一个示例中。
2. 新示例放在 `examples/<chapter>-<topic>/`，章节号使用两位数字，知识点文件使用 kebab-case，例如 `move-copy-clone.rs`。01–07 对应七条主线，08 是补充篇；章内按 P0 → P1 组织示例。
3. 每个示例提供 `pub fn main()`，目标名为 `<chapter>-<concept>`，能通过 `cargo run --locked --example <target>` 独立运行。章节入口通过 `#[path]` 导入同一份代码，保留原有 `cargo run --locked --bin <chapter>-<topic>` 命令；不复制教学逻辑。
4. 文件开头用简短中文说明问题、输入与状态变化、结果证据、改一处代码的观察点、边界和运行命令；其余注释解释“为什么”，不要逐行复述代码。术语和注释风格应与相邻示例保持一致。
5. 输出应简洁、稳定，并能直观体现当前语言特性。
6. 优先使用最小代码展示概念，不为教程示例引入不必要的抽象、依赖或工程化结构。
7. 同一章节的补充代码放在该章的示例目录并接入章节聚合入口，不创建未编号的平级 binary。异步示例独立创建 `current_thread` runtime；同步章节入口顺序调用，避免嵌套 runtime。

## 无法编译或会 panic 的教学代码

某些 Rust 规则需要通过错误示例说明。此类代码不得破坏整个仓库的可运行性：

- 将无法编译的代码注释掉，并紧邻说明预期错误及原因。
- 将故意 panic 的代码注释掉，明确标注触发条件。
- 同时提供可运行的正确写法，便于读者对照。
- 不要通过 `allow` 属性掩盖与教学重点相关的编译器诊断。

## 代码风格

- 遵循当前稳定版 Rust 和项目的 Rust 2021 edition。
- 使用 `rustfmt` 的默认格式，不添加无关的格式化配置。
- 变量名应能表达示例中的角色；只有非常局部且含义明确时才使用单字母变量。
- 示例应保持自包含。只有当多个示例确实共享非教学性逻辑时，才考虑提取公共模块。
- 修改现有示例时保持改动聚焦，不顺带重构其他示例。

## 验证要求

提交前至少执行：

```bash
cargo fmt --check
cargo check --locked --all-targets
cargo test --locked --all-targets
```

每个 `[[example]]` 设置 `test = true`，确保其中的单元测试实际执行。新增或修改示例后，还要实际运行独立示例及对应章节入口并检查输出：

```bash
cargo run --locked --example <example-name>
cargo run --locked --bin <chapter-name>
```

如果增加了包含分支或数据转换的辅助函数，应补充单元测试并运行 `cargo test`。

## 完成标准

- 示例主题单一且学习目标清晰。
- 示例可以独立编译和运行。
- 代码输出与注释所描述的行为一致。
- 错误示例不会影响 `cargo check --locked --all-targets`。
- 未引入与当前语言特性无关的依赖或改动。
