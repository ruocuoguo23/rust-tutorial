// 问题：unsafe fn 的调用者与 unsafe block 的实现者分别保证什么？
// 输入与状态：调用者提供有效 u8 的指针，函数在局部 unsafe block 中读取。
// 结果证据：读取到 7；测试只读取有效的 0、7、255。
// 改一处：删除函数内 unsafe block，观察 deny(unsafe_op_in_unsafe_fn) 的诊断。
// 边界：unsafe fn 声明调用契约，不自动证明内部操作；不能用无效指针试错。
// 运行：cargo run --locked --example 10-unsafe-call

/// # Safety
/// pointer 必须指向可读、已初始化且仍存活的 u8，本次读取没有冲突写入。
#[deny(unsafe_op_in_unsafe_fn)]
unsafe fn read_byte(pointer: *const u8) -> u8 {
    // SAFETY: 调用者契约保证本次读取的有效性。
    unsafe { pointer.read() }
}

pub fn main() {
    let value = 7_u8;
    // SAFETY: 指针来自仍有效的共享引用，读取期间没有写入者。
    assert_eq!(unsafe { read_byte(&value) }, 7);
    println!("unsafe caller contract -> local block -> byte {value}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_live_bytes_including_zero_and_maximum() {
        for value in [0, 7, u8::MAX] {
            // SAFETY: value 在本次读取期间有效，且无写入者。
            assert_eq!(unsafe { read_byte(&value) }, value);
        }
    }
}
