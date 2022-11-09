use std::collections::VecDeque;

use super::Graph;

/// Returns breadth first search trace.
pub fn bfs<G: Graph>(g: &G, from: usize, to: usize) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut depth = vec![None; g.size()];

    queue.push_back(from);
    depth[from] = Some(0);

    while let Some(current) = queue.pop_front() {
        let current_depth = match depth[current] {
            Some(x) => x,
            None => continue,
        };

        g.neighbours(current).into_iter().for_each(|i| {
            if depth[i].is_none() {
                depth[i] = Some(current_depth + 1);
                queue.push_back(i)
            }
        });
    }

    depth[to]
}

#[cfg(test)]
pub mod tests {
    use super::bfs;
    use crate::bfs::test_utils;

    #[test]
    fn simple_correctness_test() {
        test_utils::simple_correctness_test(bfs)
    }

    #[test]
    fn cubic_graph_test() {
        test_utils::cubic_graph_test(bfs)
    }
}
