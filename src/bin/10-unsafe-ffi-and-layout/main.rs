#[path = "../../../examples/10-unsafe-ffi-and-layout/raw-pointer-access.rs"]
mod raw_pointer_access;

#[path = "../../../examples/10-unsafe-ffi-and-layout/unsafe-call.rs"]
mod unsafe_call;

#[path = "../../../examples/10-unsafe-ffi-and-layout/raw-pointer-safe-wrapper.rs"]
mod raw_pointer_safe_wrapper;

#[path = "../../../examples/10-unsafe-ffi-and-layout/ffi-handle.rs"]
mod ffi_handle;

fn main() {
    raw_pointer_access::main();
    unsafe_call::main();
    raw_pointer_safe_wrapper::main();
    ffi_handle::main();
}
