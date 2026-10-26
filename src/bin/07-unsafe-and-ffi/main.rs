#[path = "../../../examples/07-unsafe-and-ffi/raw-pointer-safe-wrapper.rs"]
mod raw_pointer_safe_wrapper;

#[path = "../../../examples/07-unsafe-and-ffi/ffi-handle.rs"]
mod ffi_handle;

fn main() {
    raw_pointer_safe_wrapper::main();
    ffi_handle::main();
}
