#[path = "../../../examples/06-smart-pointers-and-resource-management/box-basics.rs"]
mod box_basics;

#[path = "../../../examples/06-smart-pointers-and-resource-management/rc-sharing.rs"]
mod rc_sharing;

#[path = "../../../examples/06-smart-pointers-and-resource-management/weak-lifecycle.rs"]
mod weak_lifecycle;

#[path = "../../../examples/06-smart-pointers-and-resource-management/arc-sharing.rs"]
mod arc_sharing;

#[path = "../../../examples/06-smart-pointers-and-resource-management/cell-counter.rs"]
mod cell_counter;

#[path = "../../../examples/06-smart-pointers-and-resource-management/refcell-borrow.rs"]
mod refcell_borrow;

#[path = "../../../examples/06-smart-pointers-and-resource-management/drop-raii.rs"]
mod drop_raii;

#[path = "../../../examples/06-smart-pointers-and-resource-management/deref-coercion.rs"]
mod deref_coercion;

#[path = "../../../examples/06-smart-pointers-and-resource-management/box-dst.rs"]
pub mod box_dst;

#[path = "../../../examples/06-smart-pointers-and-resource-management/interior-mutability.rs"]
pub mod interior_mutability;

#[path = "../../../examples/06-smart-pointers-and-resource-management/shared-ownership.rs"]
mod shared_ownership;

fn main() {
    box_basics::main();
    rc_sharing::main();
    weak_lifecycle::main();
    arc_sharing::main();
    cell_counter::main();
    refcell_borrow::main();
    drop_raii::main();
    deref_coercion::main();
    box_dst::main();
    interior_mutability::main();
    shared_ownership::main();
}
