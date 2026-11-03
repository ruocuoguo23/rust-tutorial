#[path = "../../../examples/05-collections-and-borrowed-views/string-view.rs"]
mod string_view;

#[path = "../../../examples/05-collections-and-borrowed-views/slice-view.rs"]
mod slice_view;

#[path = "../../../examples/05-collections-and-borrowed-views/slice-mutation.rs"]
mod slice_mutation;

#[path = "../../../examples/05-collections-and-borrowed-views/vec-growth.rs"]
mod vec_growth;

#[path = "../../../examples/05-collections-and-borrowed-views/boxed-slice.rs"]
mod boxed_slice;

#[path = "../../../examples/05-collections-and-borrowed-views/strings-utf8.rs"]
mod strings_utf8;

#[path = "../../../examples/05-collections-and-borrowed-views/vec-slice.rs"]
mod vec_slice;

#[path = "../../../examples/05-collections-and-borrowed-views/hashmap-entry.rs"]
mod hashmap_entry;

#[path = "../../../examples/05-collections-and-borrowed-views/cow.rs"]
mod cow;

fn main() {
    string_view::main();
    slice_view::main();
    slice_mutation::main();
    vec_growth::main();
    boxed_slice::main();
    strings_utf8::main();
    vec_slice::main();
    hashmap_entry::main();
    cow::main();
}
