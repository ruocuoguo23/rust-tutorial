#[path = "../../../examples/04-memory-and-smart-pointers/box-dst.rs"]
pub mod box_dst;

#[path = "../../../examples/04-memory-and-smart-pointers/shared-ownership.rs"]
mod shared_ownership;

#[path = "../../../examples/04-memory-and-smart-pointers/interior-mutability.rs"]
pub mod interior_mutability;

#[path = "../../../examples/04-memory-and-smart-pointers/drop-raii.rs"]
mod drop_raii;

#[path = "../../../examples/04-memory-and-smart-pointers/deref-coercion.rs"]
mod deref_coercion;

fn main() {
    box_dst::p0();
    shared_ownership::main();
    interior_mutability::p0();
    drop_raii::main();
    deref_coercion::main();
    interior_mutability::p1();
    box_dst::p1();
}
