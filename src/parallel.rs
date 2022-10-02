use crate::sequential::{self, partition};
use rayon::ThreadPoolBuildError;

pub fn quick_sort<T: Ord + Send>(slice: &mut [T], block: usize) {
    if slice.len() <= block {
        sequential::quick_sort(slice);
        return;
    }

    let index = partition(slice);
    let (left, right) = slice.split_at_mut(index);
    rayon::join(
        || quick_sort(left, block),
        || quick_sort(&mut right[1..], block),
    );
}

pub fn quick_sort_with_thread_pool<T: Ord + Send>(
    slice: &mut [T],
    num_threads: usize,
    block: usize,
) -> Result<(), ThreadPoolBuildError> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .map(|pool| {
            pool.install(|| quick_sort(slice, block));
        })
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn correctness_test() {
        crate::test_utils::correctness_test(|v| super::quick_sort(v, 1));
    }
}
