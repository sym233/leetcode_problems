impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut mat = vec![vec![]; n + 1];
        for edge in edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            if Self::dfs(&mat, &mut vec![false; n + 1], a, b) {
                return edge;
            }
            mat[a].push(b);
            mat[b].push(a);
        }
        panic!("no solution")
    }

    fn dfs(mat: &Vec<Vec<usize>>, visited: &mut Vec<bool>, start: usize, end: usize) -> bool {
        if start == end {
            return true;
        }
        for &node in mat[start].iter() {
            if node == end {
                return true;
            }
            if visited[node] {
                continue;
            }
            visited[node] = true;
            let res = Self::dfs(mat, visited, node, end);
            if res {
                return true;
            }
        }
        false
    }
}
