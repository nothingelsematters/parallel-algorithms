use std::collections::VecDeque;

use super::AdjacencyMatrix;

/// Precondition: matrix dimensions are the same.
/// Returns breadth first search trace.
pub fn bfs(adjacency_matrix: &AdjacencyMatrix) -> Vec<usize> {
    if adjacency_matrix.is_empty() {
        return Vec::new();
    }

    let mut queue = VecDeque::new();
    let mut visited = vec![false; adjacency_matrix.len()];
    let mut trace = Vec::with_capacity(adjacency_matrix.len());

    queue.push_back(0);

    while let Some(current) = queue.pop_front() {
        if visited[current] {
            continue;
        }

        trace.push(current);
        visited[current] = true;

        adjacency_matrix[current]
            .iter()
            .enumerate()
            .filter(|(_, x)| **x)
            .map(|(i, _)| i)
            .filter(|i| !visited[*i])
            .for_each(|i| queue.push_back(i));
    }

    trace
}

#[cfg(test)]
pub mod tests {
    use super::bfs;

    #[test]
    fn simple_correctness_test() {
        crate::bfs::test_utils::simple_correctness_test(bfs)
    }
}
