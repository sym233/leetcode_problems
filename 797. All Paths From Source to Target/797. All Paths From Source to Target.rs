use std::collections::VecDeque;
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = graph.len();
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut q: VecDeque<Vec<i32>> = VecDeque::from(vec![vec![0]]);
        while let Some(path) = q.pop_front() {
            if let Some(&last) = path.last() {
                if last as usize == n - 1 {
                    res.push(path);
                    continue;
                }
                for &node in &graph[last as usize] {
                    let mut path = path.clone();
                    path.push(node);
                    q.push_back(path);
                }
            }
        }
        return res;
    }
}
