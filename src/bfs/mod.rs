pub mod parallel;
pub mod sequential;
#[cfg(test)]
mod test_utils;

pub type AdjacencyMatrix = Vec<Vec<bool>>;
