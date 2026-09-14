// An opaque handle: a foreign caller must not inspect its Rust layout.
struct Handle {
    value: u32,
}

extern "C" fn handle_new(value: u32) -> *mut Handle {
    Box::into_raw(Box::new(Handle { value }))
}

/// # Safety
/// Except for null, handle must be the unmodified live pointer returned by handle_new.
/// The caller transfers its unique release responsibility here, exactly once.
/// No references or concurrent accesses may remain when the allocation is freed.
unsafe extern "C" fn handle_free(handle: *mut Handle) {
    if handle.is_null() {
        return; // This teaching protocol explicitly accepts null as a no-op.
    }
    // SAFETY: the caller guarantees the matching Box allocation and exclusive release rights.
    unsafe {
        drop(Box::from_raw(handle));
    }
}

pub fn run() {
    let handle = handle_new(7);
    // SAFETY: this live handle has not been freed or handed to another owner.
    let value = unsafe { (*handle).value };
    assert_eq!(value, 7);
    // SAFETY: this is the sole release; only a copied integer remains after the read.
    unsafe {
        handle_free(handle);
    }
    println!("Opaque handle: allocated -> read {value} -> released once");
    // Never read or free handle again. Null/alignment checks alone cannot validate a foreign pointer.
    // extern "C" selects an ABI, not a network format or an ownership protocol.
    // These functions are called from Rust; a real exported library needs cross-language tests.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_round_trip_preserves_values_and_releases_once() {
        for value in [0, 7, u32::MAX] {
            let handle = handle_new(value);
            // SAFETY: fresh Box allocation, still live and exclusively managed by this test.
            assert_eq!(unsafe { (*handle).value }, value);
            // SAFETY: no references remain and this is the only release.
            unsafe {
                handle_free(handle);
            }
        }
    }

    #[test]
    fn null_release_is_a_protocol_noop() {
        // SAFETY: this protocol explicitly permits null.
        unsafe {
            handle_free(std::ptr::null_mut());
        }
    }
}
