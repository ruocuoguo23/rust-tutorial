#[path = "../../../examples/03-traits-generics-and-dispatch/associated-types.rs"]
mod associated_types;
#[path = "../../../examples/03-traits-generics-and-dispatch/derive-equality.rs"]
mod derive_equality;
#[path = "../../../examples/03-traits-generics-and-dispatch/newtype-conversions.rs"]
mod newtype_conversions;
#[path = "../../../examples/03-traits-generics-and-dispatch/trait-dispatch.rs"]
mod trait_dispatch;

fn main() {
    println!("P0: Generic / Trait Bound / impl Trait / dyn Trait");
    trait_dispatch::main();
    println!("P0: Associated Type");
    associated_types::main();
    println!("P1: Newtype / From / Into / fallible conversion / derive");
    newtype_conversions::main();
    derive_equality::main();
}
