mod borrowing;
mod ownership;

fn main() {
    println!("P0: Move / Copy / Clone and borrowed APIs");
    ownership::run();
    println!("P0: Borrow / NLL / Lifetime");
    borrowing::run();
    println!("P1: Partial move / take / replace / reborrow / 'static");
    ownership::partial_move_and_take();
    borrowing::reborrow_and_static();
}
