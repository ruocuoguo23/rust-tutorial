#[path = "../../../examples/03-functions-generics-and-traits/associated-types.rs"]
mod associated_types;
#[path = "../../../examples/03-functions-generics-and-traits/closure-fn.rs"]
mod closure_fn;
#[path = "../../../examples/03-functions-generics-and-traits/closure-fn-mut.rs"]
mod closure_fn_mut;
#[path = "../../../examples/03-functions-generics-and-traits/closure-fn-once.rs"]
mod closure_fn_once;
#[path = "../../../examples/03-functions-generics-and-traits/closure-traits.rs"]
mod closure_traits;
#[path = "../../../examples/03-functions-generics-and-traits/derive-equality.rs"]
mod derive_equality;
#[path = "../../../examples/03-functions-generics-and-traits/dyn-borrowed.rs"]
mod dyn_borrowed;
#[path = "../../../examples/03-functions-generics-and-traits/dyn-collection.rs"]
mod dyn_collection;
#[path = "../../../examples/03-functions-generics-and-traits/generic-basics.rs"]
mod generic_basics;
#[path = "../../../examples/03-functions-generics-and-traits/impl-trait-argument.rs"]
mod impl_trait_argument;
#[path = "../../../examples/03-functions-generics-and-traits/impl-trait-return.rs"]
mod impl_trait_return;
#[path = "../../../examples/03-functions-generics-and-traits/iterator-ownership.rs"]
mod iterator_ownership;
#[path = "../../../examples/03-functions-generics-and-traits/lazy-iterators.rs"]
mod lazy_iterators;
#[path = "../../../examples/03-functions-generics-and-traits/newtype-conversions.rs"]
mod newtype_conversions;
#[path = "../../../examples/03-functions-generics-and-traits/trait-bound.rs"]
mod trait_bound;
#[path = "../../../examples/03-functions-generics-and-traits/trait-dispatch.rs"]
mod trait_dispatch;

fn main() {
    println!("P0: Generic / Trait Bound / impl Trait / dyn Trait / Associated Type");
    generic_basics::main();
    trait_bound::main();
    impl_trait_argument::main();
    impl_trait_return::main();
    dyn_borrowed::main();
    dyn_collection::main();
    associated_types::main();
    println!("P1: Closure captures / Iterator steps");
    closure_fn::main();
    closure_fn_mut::main();
    closure_fn_once::main();
    lazy_iterators::main();
    iterator_ownership::main();
    println!("P1: Newtype / From / Into / derive");
    newtype_conversions::main();
    derive_equality::main();
    println!("P1: Combined comparisons after the individual examples");
    closure_traits::main();
    trait_dispatch::main();
}
