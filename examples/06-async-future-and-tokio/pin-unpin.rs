// 问题：移动 Pin 指针句柄，是否也移动了它保护的值？
// 输入与状态：将 !Unpin 的 Marker 放入 Box::pin，记录地址，再移动 Box 句柄。
// 结果证据：断言句柄移动前后的堆上地址相同；u8: Unpin 可以安全地取回 Box。
// 改一处：取消注释 Pin::into_inner(moved_handle)，观察 Marker: !Unpin 导致编译拒绝。
// 边界：PhantomPinned 只演示类型约束；该 Marker 并非自引用结构，Pin 也不会自动延长生命周期。
// 运行：cargo run --locked --example 06-pin-unpin

use std::marker::PhantomPinned;

fn pin_and_unpin() {
    struct Marker {
        value: u8,
        _pin: PhantomPinned,
    }
    let pinned = Box::pin(Marker {
        value: 7,
        _pin: PhantomPinned,
    });
    let address = &*pinned as *const Marker;
    let moved_handle = pinned;
    assert!(std::ptr::eq(address, &*moved_handle));
    println!(
        "Pin: handle moved, pointee unchanged, value={}",
        moved_handle.value
    );
    // std::pin::Pin::into_inner(moved_handle); // E0277: Marker is !Unpin.
    let unpinned = std::pin::Pin::into_inner(Box::pin(7_u8)); // u8: Unpin.
    assert_eq!(*unpinned, 7);
    // Marker is not self-referential; PhantomPinned demonstrates the type constraint only.
}

pub fn main() {
    println!("P1: Pin / Unpin");
    pin_and_unpin();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinned_handle_moves_without_moving_its_pointee() {
        pin_and_unpin();
    }
}
