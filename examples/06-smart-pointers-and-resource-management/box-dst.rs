// 问题：学过 Box 后，拥有数组与拥有动态长度切片的表示有何区别？
// 输入与状态：Box<[i32; 3]> 转移 owner；Vec 转为 Box<[i32]> 后借作 slice。
// 结果证据：数组内容仍为 [10, 20, 30]，slice 长度为 3。
// 改一处：在 move 后使用 boxed，观察 E0382；改数组长度观察 DST 元数据。
// 边界：move 不克隆堆分配；slice 借用不能超过拥有它的 Box。
// 运行：cargo run --locked --example 06-box-dst

pub fn p0() {
    println!("P0: Stack / Heap / Box");
    let boxed = Box::new([10, 20, 30]);
    let moved_box = boxed; // Move the owner; this does not clone the allocation.
    println!("Box owns an array: {moved_box:?}");
}

pub fn p1() {
    println!("P1: DST (Dynamically Sized Type)");
    let fixed: Box<[i32]> = vec![10, 20, 30].into_boxed_slice();
    let view: &[i32] = &fixed;
    println!("Owned Box<[i32]>, borrowed slice length: {}", view.len());
}

pub fn main() {
    p0();
    p1();
}
