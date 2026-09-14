mod ffi;

fn get_two_mut<T>(values: &mut [T], left: usize, right: usize) -> Option<(&mut T, &mut T)> {
    if left == right || left >= values.len() || right >= values.len() {
        return None;
    }

    let pointer = values.as_mut_ptr();
    // SAFETY: the input slice guarantees live, aligned, initialized, exclusively borrowed storage.
    // Both indices are in bounds and distinct; returned borrows cannot outlive the input borrow.
    unsafe { Some((&mut *pointer.add(left), &mut *pointer.add(right))) }
}

fn get_two_mut_safe<T>(values: &mut [T], left: usize, right: usize) -> Option<(&mut T, &mut T)> {
    if left == right || left >= values.len() || right >= values.len() {
        return None;
    }
    if left < right {
        let (before, after) = values.split_at_mut(right);
        Some((&mut before[left], &mut after[0]))
    } else {
        let (before, after) = values.split_at_mut(left);
        Some((&mut after[0], &mut before[right]))
    }
}

fn main() {
    println!("P0: Raw Pointer / Safety Invariant / Safe Abstraction");
    let mut values = [10, 20, 30];
    let (left, right) = get_two_mut(&mut values, 0, 2).unwrap();
    *left += 1;
    *right += 2;

    println!("After disjoint mutation: {values:?}");
    println!(
        "Same index rejected: {}",
        get_two_mut(&mut values, 1, 1).is_none()
    );
    let (left, right) = get_two_mut_safe(&mut values, 2, 0).unwrap();
    *left += 1;
    *right += 1;
    assert_eq!(values, [12, 20, 33]);
    println!("Safe split_at_mut alternative: {values:?}");
    println!("P1: FFI (Foreign Function Interface) ownership and release");
    ffi::run();
}

#[cfg(test)]
mod tests {
    use super::{get_two_mut, get_two_mut_safe};

    #[test]
    fn returns_distinct_mutable_references() {
        let mut values = [1, 2, 3];
        let (left, right) = get_two_mut(&mut values, 0, 2).unwrap();
        *left = 10;
        *right = 30;
        assert_eq!(values, [10, 2, 30]);
    }

    #[test]
    fn rejects_equal_or_out_of_bounds_indices() {
        let mut values = [1, 2, 3];
        assert!(get_two_mut(&mut values, 1, 1).is_none());
        assert!(get_two_mut(&mut values, 0, 3).is_none());
        assert!(get_two_mut(&mut values, 3, 0).is_none());
        assert!(get_two_mut::<u8>(&mut [], 0, 1).is_none());
        assert!(get_two_mut(&mut [1], 0, 0).is_none());
    }

    #[test]
    fn safe_and_unsafe_versions_agree_for_both_index_orders_and_invalid_inputs() {
        for left in 0..5 {
            for right in 0..5 {
                let mut unsafe_values = [1, 2, 3];
                let mut safe_values = unsafe_values;
                let actual = get_two_mut(&mut unsafe_values, left, right);
                let expected = get_two_mut_safe(&mut safe_values, left, right);
                assert_eq!(actual.is_some(), expected.is_some());
                if let Some((left, right)) = actual {
                    *left = 10;
                    *right = 20;
                }
                if let Some((left, right)) = expected {
                    *left = 10;
                    *right = 20;
                }
                assert_eq!(unsafe_values, safe_values);
            }
        }
    }

    #[test]
    fn distinct_zero_sized_elements_are_supported() {
        assert!(get_two_mut(&mut [(), ()], 0, 1).is_some());
    }
}
