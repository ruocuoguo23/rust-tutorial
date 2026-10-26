#[path = "../../../examples/08-collections-closures-and-engineering/strings-utf8.rs"]
mod strings_utf8;

#[path = "../../../examples/08-collections-closures-and-engineering/vec-slice.rs"]
mod vec_slice;

#[path = "../../../examples/08-collections-closures-and-engineering/hashmap-entry.rs"]
mod hashmap_entry;

#[path = "../../../examples/08-collections-closures-and-engineering/cow.rs"]
mod cow;

#[path = "../../../examples/08-collections-closures-and-engineering/closure-traits.rs"]
mod closure_traits;

#[path = "../../../examples/08-collections-closures-and-engineering/lazy-iterators.rs"]
mod lazy_iterators;

#[path = "../../../examples/08-collections-closures-and-engineering/macro-evaluation.rs"]
mod macro_evaluation;

fn main() {
    strings_utf8::main();
    vec_slice::main();
    hashmap_entry::main();
    cow::main();
    closure_traits::main();
    lazy_iterators::main();
    macro_evaluation::main();
}
