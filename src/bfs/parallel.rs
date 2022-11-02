use rayon::ThreadPoolBuildError;

use super::AdjacencyMatrix;

pub fn bfs(_adjacency_matrix: &AdjacencyMatrix) -> Vec<usize> {
    vec![]
}

pub fn bfs_with_thread_pool(
    adjacency_matrix: &AdjacencyMatrix,
    num_threads: usize,
) -> Result<Vec<usize>, ThreadPoolBuildError> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .map(|pool| pool.install(|| bfs(adjacency_matrix)))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::bfs::test_utils;

    #[test]
    fn simple_correctness_test() {
        test_utils::simple_correctness_test(bfs)
    }

    #[test]
    fn pooled_simple_correctness_test() {
        test_utils::simple_correctness_test(|g| {
            bfs_with_thread_pool(g, 4).expect("Successful pool creation")
        })
    }
}
