// Check that every element is present only once on an sorted slice.
// (for all i1, i2 in 0..arr.len())(arr[i1] == arr[i2] --> i1 == i2)
// Because the slice is sorted, we can just do a linear search to check
// if there are duplicate elements.
pub fn has_unique_elements<T: Ord>(arr: &[T]) -> bool {
    let len = arr.len();

    // this is needed otherwise (len - 1) would underflow
    if len == 0 {
        return true;
    }

    for i in 0..(len - 1) {
        if arr[i] == arr[i + 1] {
            return false;
        }
    }

    true
}
