// 问题：通过 C ABI 传递不透明句柄时，谁承担唯一释放责任？
// 输入与状态：Box 分配 Handle(7)，转为原始指针读取数值，再由匹配的 handle_free 释放一次。
// 结果证据：读取值为 7；测试覆盖 0、7、u32::MAX，以及协议允许的 null 释放。
// 改一处：把 handle_new 的 7 改成 0 并调整断言，观察分配和释放协议不变。
// 边界：这里只由 Rust 调用 Rust 的 extern C 函数；没有真实 C 链接测试，null 检查也不能验证任意外部指针。
// 运行：cargo run --locked --example 07-ffi-handle

// An opaque handle: a foreign caller must not inspect its Rust layout.
struct Handle {
    value: u32,
}

extern "C" fn handle_new(value: u32) -> *mut Handle {
    Box::into_raw(Box::new(Handle { value }))
}

/// # Safety
/// Except for null, handle must be the unmodified live pointer returned by handle_new.
/// The caller transfers its unique release responsibility here, exactly once.
/// No references or concurrent accesses may remain when the allocation is freed.
unsafe extern "C" fn handle_free(handle: *mut Handle) {
    if handle.is_null() {
        return; // This teaching protocol explicitly accepts null as a no-op.
    }
    // SAFETY: the caller guarantees the matching Box allocation and exclusive release rights.
    unsafe {
        drop(Box::from_raw(handle));
    }
}

pub fn main() {
    println!("P1: FFI (Foreign Function Interface) ownership and release");
    let handle = handle_new(7);
    // SAFETY: this live handle has not been freed or handed to another owner.
    let value = unsafe { (*handle).value };
    assert_eq!(value, 7);
    // SAFETY: this is the sole release; only a copied integer remains after the read.
    unsafe {
        handle_free(handle);
    }
    println!("Opaque handle: allocated -> read {value} -> released once");
    // Never read or free handle again. Null/alignment checks alone cannot validate a foreign pointer.
    // extern "C" selects an ABI, not a network format or an ownership protocol.
    // These functions are called from Rust; a real exported library needs cross-language tests.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_round_trip_preserves_values_and_releases_once() {
        for value in [0, 7, u32::MAX] {
            let handle = handle_new(value);
            // SAFETY: fresh Box allocation, still live and exclusively managed by this test.
            assert_eq!(unsafe { (*handle).value }, value);
            // SAFETY: no references remain and this is the only release.
            unsafe {
                handle_free(handle);
            }
        }
    }

    #[test]
    fn null_release_is_a_protocol_noop() {
        // SAFETY: this protocol explicitly permits null.
        unsafe {
            handle_free(std::ptr::null_mut());
        }
    }
}
