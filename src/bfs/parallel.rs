use rayon::prelude::*;
use rayon::ThreadPoolBuildError;
use std::sync::atomic::AtomicBool;
use std::sync::{atomic::Ordering, Arc};

use super::Graph;

pub fn bfs<'a, G: Graph>(g: Arc<&'a G>, from: usize, to: usize) -> Option<usize>
where
    Arc<&'a G>: Send + Sync,
{
    let mut frontier = vec![from];
    let mut depth = vec![None; g.size()];
    depth[from] = Some(0);

    let mut new_frontier = Vec::new();
    let taken = {
        let mut vec = Vec::with_capacity(g.size());
        for _ in 0..g.size() {
            vec.push(AtomicBool::new(false));
        }
        vec
    };

    while !frontier.is_empty() {
        let degree = frontier
            .par_iter()
            .map(|v| g.neighbours(*v).len())
            .fold(Vec::new, |mut v, i| {
                v.push(v.last().unwrap_or(&0) + i);
                v
            })
            .reduce(Vec::new, |mut l, mut r| {
                let last_l = l.last().unwrap_or(&0);
                r.iter_mut().for_each(|x| *x += last_l);
                l.append(&mut r);
                l
            });

        let new_frontier_len = *degree.last().unwrap();
        if new_frontier_len > new_frontier.len() {
            for _ in 0..new_frontier_len - new_frontier.len() {
                new_frontier.push(None);
            }
        }

        frontier.into_par_iter().enumerate().for_each(|(i, v)| {
            let v_depth = depth[v].unwrap();
            let i_degree = if i == 0 { 0 } else { degree[i - 1] };

            g.neighbours(v)
                .into_iter()
                .enumerate()
                .filter(|(_, u)| {
                    taken[*u]
                        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                        .is_ok()
                })
                .for_each(|(j, u)| {
                    #[allow(clippy::cast_ref_to_mut)]
                    unsafe {
                        *(&new_frontier[i_degree + j] as *const _ as *mut _) = Some(u);
                        *(&depth[u] as *const _ as *mut _) = Some(v_depth + 1);
                    }
                })
        });

        frontier = new_frontier.iter_mut().filter_map(|x| x.take()).collect();
    }

    depth[to]
}

pub fn bfs_with_thread_pool<'a, G: Graph + 'a>(
    g: &'a G,
    from: usize,
    to: usize,
    num_threads: usize,
) -> Result<Option<usize>, ThreadPoolBuildError>
where
    Arc<&'a G>: Send + Sync,
{
    let g = Arc::new(g);

    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .map(|pool| pool.install(|| bfs(g, from, to)))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::bfs::CubicGraph;

    #[test]
    fn cubic_graph_test() {
        let side = 100;

        assert_eq!(
            Some((side - 1) * 3),
            bfs_with_thread_pool(&CubicGraph::new(side), 0, side.pow(3) - 1, 4)
                .expect("Successful pool creation")
        )
    }
}
