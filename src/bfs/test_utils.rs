use super::AdjacencyMatrix;

pub(crate) fn simple_correctness_test<F>(bfs: F)
where
    F: Fn(&AdjacencyMatrix) -> Vec<usize>,
{
    let graph = vec![
        vec![false, true, false, true],
        vec![true, false, true, false],
        vec![false, true, false, true],
        vec![true, true, true, false],
    ];

    assert_eq!(vec![0, 1, 3, 2], bfs(&graph))
}
