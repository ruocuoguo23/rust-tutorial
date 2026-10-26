#[path = "../../../examples/01-ownership-and-borrowing/borrow-nll.rs"]
mod borrow_nll;
#[path = "../../../examples/01-ownership-and-borrowing/lifetimes.rs"]
mod lifetimes;
#[path = "../../../examples/01-ownership-and-borrowing/move-copy-clone.rs"]
mod move_copy_clone;
#[path = "../../../examples/01-ownership-and-borrowing/partial-move-take-replace.rs"]
mod partial_move_take_replace;
#[path = "../../../examples/01-ownership-and-borrowing/reborrow-static.rs"]
mod reborrow_static;

fn main() {
    println!("P0: Move / Copy / Clone and borrowed APIs");
    move_copy_clone::main();
    println!("P0: Borrow / NLL / Lifetime");
    borrow_nll::main();
    lifetimes::main();
    println!("P1: Partial move / take / replace / reborrow / 'static");
    partial_move_take_replace::main();
    reborrow_static::main();
}
