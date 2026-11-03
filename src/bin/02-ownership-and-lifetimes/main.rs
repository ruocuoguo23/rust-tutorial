#[path = "../../../examples/02-ownership-and-lifetimes/move-value.rs"]
mod move_value;

#[path = "../../../examples/02-ownership-and-lifetimes/copy-value.rs"]
mod copy_value;

#[path = "../../../examples/02-ownership-and-lifetimes/clone-value.rs"]
mod clone_value;

#[path = "../../../examples/02-ownership-and-lifetimes/borrow-nll.rs"]
mod borrow_nll;

#[path = "../../../examples/02-ownership-and-lifetimes/lifetimes.rs"]
mod lifetimes;

#[path = "../../../examples/02-ownership-and-lifetimes/move-copy-clone.rs"]
mod move_copy_clone;

#[path = "../../../examples/02-ownership-and-lifetimes/partial-move-take-replace.rs"]
mod partial_move_take_replace;

#[path = "../../../examples/02-ownership-and-lifetimes/reborrow-static.rs"]
mod reborrow_static;

fn main() {
    println!("P0: Move, Copy and Clone one at a time");
    move_value::main();
    copy_value::main();
    clone_value::main();
    println!("P0: Borrow, NLL and Lifetime");
    borrow_nll::main();
    lifetimes::main();
    println!("P0: Custom types and ownership API combination");
    move_copy_clone::main();
    println!("P1: Partial move, take, replace, reborrow and 'static");
    partial_move_take_replace::main();
    reborrow_static::main();
}
