use super::{AdjacencyMatrixGraph, CubicGraph};

pub(crate) fn simple_correctness_test<F>(bfs: F)
where
    F: Fn(&AdjacencyMatrixGraph, usize, usize) -> Option<usize>,
{
    let adjacency_matrix = vec![
        vec![false, true, false, true],
        vec![true, false, true, false],
        vec![false, true, false, true],
        vec![true, true, true, false],
    ];
    let graph = AdjacencyMatrixGraph::new(adjacency_matrix).unwrap();

    assert_eq!(Some(2), bfs(&graph, 0, 2))
}

pub(crate) fn cubic_graph_test<F>(bfs: F)
where
    F: Fn(&CubicGraph, usize, usize) -> Option<usize>,
{
    assert_eq!(Some(27), bfs(&CubicGraph::new(10), 0, 10usize.pow(3) - 1))
}
