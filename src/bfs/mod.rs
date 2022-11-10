pub mod parallel;
pub mod sequential;
#[cfg(test)]
mod test_utils;

pub trait Graph {
    fn size(&self) -> usize;

    fn neighbours(&self, index: usize) -> Vec<usize>;

    fn neighbours_size(&self, index: usize) -> usize;
}

pub struct AdjacencyMatrixGraph {
    adjacency_matrix: Vec<Vec<bool>>,
}

impl AdjacencyMatrixGraph {
    pub fn new(adjacency_matrix: Vec<Vec<bool>>) -> Option<AdjacencyMatrixGraph> {
        for i in adjacency_matrix.iter() {
            if i.len() != adjacency_matrix.len() {
                return None;
            }
        }

        Some(AdjacencyMatrixGraph { adjacency_matrix })
    }
}

impl Graph for AdjacencyMatrixGraph {
    fn size(&self) -> usize {
        self.adjacency_matrix.len()
    }

    fn neighbours(&self, index: usize) -> Vec<usize> {
        self.adjacency_matrix[index]
            .iter()
            .enumerate()
            .filter(|(_, x)| **x)
            .map(|(i, _)| i)
            .collect()
    }

    fn neighbours_size(&self, index: usize) -> usize {
        self.adjacency_matrix[index].iter().filter(|x| **x).count()
    }
}

pub struct EdgesGraph {
    edges: Vec<Vec<usize>>,
}

impl EdgesGraph {
    pub fn new(edges: Vec<Vec<usize>>) -> Option<EdgesGraph> {
        Some(EdgesGraph { edges })
    }
}

impl Graph for EdgesGraph {
    fn size(&self) -> usize {
        self.edges.len()
    }

    fn neighbours(&self, index: usize) -> Vec<usize> {
        self.edges[index].clone()
    }

    fn neighbours_size(&self, index: usize) -> usize {
        self.edges[index].len()
    }
}

pub struct CubicGraph {
    size: usize,
}

impl CubicGraph {
    pub fn new(size: usize) -> CubicGraph {
        CubicGraph { size }
    }
}

impl Graph for CubicGraph {
    fn size(&self) -> usize {
        self.size.pow(3)
    }

    fn neighbours(&self, index: usize) -> Vec<usize> {
        let i = index / self.size.pow(2);
        let j = (index - self.size.pow(2) * i) / self.size;
        let k = index - self.size.pow(2) * i - j * self.size;

        let mut neighbours = Vec::new();

        if i != 0 {
            neighbours.push(index - self.size.pow(2));
        }
        if i != self.size - 1 {
            neighbours.push(index + self.size.pow(2));
        }
        if j != 0 {
            neighbours.push(index - self.size);
        }
        if j != self.size - 1 {
            neighbours.push(index + self.size);
        }
        if k != 0 {
            neighbours.push(index - 1);
        }
        if k != self.size - 1 {
            neighbours.push(index + 1);
        }

        neighbours
    }

    fn neighbours_size(&self, index: usize) -> usize {
        let i = index / self.size.pow(2);
        let j = (index - self.size.pow(2) * i) / self.size;
        let k = index - self.size.pow(2) * i - j * self.size;

        let mut result = 0;

        if i != 0 {
            result += 1;
        }
        if i != self.size - 1 {
            result += 1;
        }
        if j != 0 {
            result += 1;
        }
        if j != self.size - 1 {
            result += 1;
        }
        if k != 0 {
            result += 1;
        }
        if k != self.size - 1 {
            result += 1;
        }

        result
    }
}
