pub fn quick_sort<T: Ord>(slice: &mut [T]) {
    if slice.is_empty() {
        return;
    }

    let index = partition(slice);
    let (left, right) = slice.split_at_mut(index);
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

pub(crate) fn partition<T: Ord>(slice: &mut [T]) -> usize {
    let pivot = slice.len() - 1;
    let mut i = 0;

    for j in 0..pivot {
        if slice[j] <= slice[pivot] {
            slice.swap(i, j);
            i += 1;
        }
    }

    slice.swap(i, pivot);
    i
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn correctness_test() {
        crate::quick_sort::test_utils::correctness_test(super::quick_sort)
    }
}
