use crate::turing::error::DuplicateError;
use std::fmt;

// Check that every element is present only once on a sorted slice.
// (for all i1, i2 in 0..arr.len())(eq(arr[i1], arr[i2]) -> i1 == i2)
// Because the slice is sorted, we can just do a linear search to check
// if there are duplicate elements.
pub fn has_unique_elements_by<T: Clone + fmt::Display + fmt::Debug>(
    arr: &[T],
    eq: impl Fn(&T, &T) -> bool,
    type_name: &str,
) -> Result<(), DuplicateError<T>> {
    let len = arr.len();

    // this is needed as (len - 1) would underflow
    if len == 0 {
        return Ok(());
    }

    for i in 0..(len - 1) {
        if eq(&arr[i], &arr[i + 1]) {
            return Err(DuplicateError::new(
                arr[i].clone(),
                arr[i + 1].clone(),
                type_name,
            ));
        }
    }

    Ok(())
}

pub fn has_unique_elements<T: Clone + Ord + fmt::Display + fmt::Debug>(
    arr: &[T],
    type_name: &str,
) -> Result<(), DuplicateError<T>> {
    has_unique_elements_by(arr, T::eq, type_name)
}
