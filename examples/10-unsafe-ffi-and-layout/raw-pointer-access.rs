// 问题：创建裸指针与通过它访问数据，有什么区别？
// 输入与状态：从有效整数 7 创建裸指针，在受证明的 unsafe block 内改为 8。
// 结果证据：原整数最终为 8。
// 改一处：只创建指针而删除解引用，观察创建本身不需要 unsafe。
// 边界：非空不等于有效；访问还要满足寿命、对齐、初始化、范围和无冲突访问。
// 运行：cargo run --locked --example 10-raw-pointer-access

pub fn main() {
    let mut value = 7_i32;
    let pointer = &mut value as *mut i32;
    // SAFETY: pointer 来自仍存活、已初始化且对齐的 value，此处没有冲突访问。
    unsafe {
        *pointer += 1;
    }
    assert_eq!(value, 8);
    println!("Raw pointer update: 7 -> {value}");
}
