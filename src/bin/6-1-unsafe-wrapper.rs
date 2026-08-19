fn get_two_mut<T>(values: &mut [T], left: usize, right: usize) -> Option<(&mut T, &mut T)> {
    if left == right || left >= values.len() || right >= values.len() {
        return None;
    }

    let pointer = values.as_mut_ptr();
    // SAFETY: both indices are in bounds and distinct, so they identify non-overlapping elements.
    unsafe { Some((&mut *pointer.add(left), &mut *pointer.add(right))) }
}

fn main() {
    let mut values = [10, 20, 30];
    let (left, right) = get_two_mut(&mut values, 0, 2).unwrap();
    *left += 1;
    *right += 2;

    println!("After disjoint mutation: {values:?}");
    println!(
        "Same index rejected: {}",
        get_two_mut(&mut values, 1, 1).is_none()
    );
}

#[cfg(test)]
mod tests {
    use super::get_two_mut;

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
    }
}
