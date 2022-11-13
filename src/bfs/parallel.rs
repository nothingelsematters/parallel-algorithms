use rayon::prelude::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use rayon::ThreadPoolBuildError;
use std::sync::atomic::AtomicUsize;
use std::sync::{atomic::Ordering, Arc};

use super::Graph;

struct AtomicBoolVec {
    vec: Vec<AtomicUsize>,
}

impl AtomicBoolVec {
    const USIZE_BITS: usize = usize::BITS as usize;

    fn new(len: usize) -> AtomicBoolVec {
        let mut vec = Vec::with_capacity(len / Self::USIZE_BITS + 1);
        for _ in 0..len / Self::USIZE_BITS + 1 {
            vec.push(AtomicUsize::new(0));
        }
        AtomicBoolVec { vec }
    }

    fn compare_and_set(&self, index: usize) -> bool {
        loop {
            let vec_index = index / Self::USIZE_BITS;
            let loaded = self.vec[vec_index].load(Ordering::Acquire);
            let bit = 1 << (index % Self::USIZE_BITS);

            if loaded & bit == 0 {
                if self.vec[vec_index]
                    .compare_exchange(loaded, loaded | bit, Ordering::Acquire, Ordering::Relaxed)
                    .is_ok()
                {
                    break true;
                }
            } else {
                break false;
            }
        }
    }
}

/// Returns the shortest path length between `from` and `to` vertices.
pub fn bfs<'a, G: Graph>(g: Arc<&'a G>, from: usize, to: usize) -> Option<usize>
where
    Arc<&'a G>: Send + Sync,
{
    let mut frontier = vec![from];
    let mut depth = vec![None; g.size()];
    depth[from] = Some(0);

    let mut new_frontier = Vec::new();
    let taken = AtomicBoolVec::new(g.size());

    while !frontier.is_empty() {
        let degree = frontier
            .par_iter()
            .map(|v| g.neighbours_size(*v))
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
            let additional = new_frontier_len - new_frontier.len();

            new_frontier.reserve(additional);
            for _ in 0..additional {
                new_frontier.push(None);
            }
        }

        frontier.into_par_iter().enumerate().for_each(|(i, v)| {
            g.neighbours(v)
                .into_iter()
                .filter(|u| taken.compare_and_set(*u))
                .enumerate()
                .for_each(|(j, u)| {
                    let i_degree = if i == 0 { 0 } else { degree[i - 1] };

                    #[allow(clippy::cast_ref_to_mut)]
                    unsafe {
                        *(&new_frontier[i_degree + j] as *const _ as *mut _) = Some(u);
                        *(&depth[u] as *const _ as *mut _) = Some(depth[v].unwrap() + 1);
                    }
                })
        });

        frontier = new_frontier
            .iter_mut()
            .take(new_frontier_len)
            .filter_map(Option::take)
            .collect();
    }

    depth[to]
}

pub fn bfs_with_thread_pool<'a, G>(
    g: &'a G,
    from: usize,
    to: usize,
    num_threads: usize,
) -> Result<Option<usize>, ThreadPoolBuildError>
where
    G: Graph + 'a,
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
    use super::bfs_with_thread_pool;
    use crate::bfs::test_utils;

    macro_rules! bfs {
        () => {
            |g, from: usize, to: usize| {
                bfs_with_thread_pool(g, from, to, 4).expect("Successful pool creation")
            }
        };
    }

    #[test]
    fn simple_correctness_test() {
        test_utils::simple_correctness_test(bfs!())
    }

    #[test]
    fn cubic_graph_test() {
        crate::bfs::test_utils::cubic_graph_test(bfs!())
    }
}
