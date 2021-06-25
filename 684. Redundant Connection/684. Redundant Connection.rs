impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut union = vec![0usize; n + 1];
        
        for edge in edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            if union[a] == 0usize {
                union[a] = a;
            }
            if union[b] == 0 {
                union[b] = Self::find_parent(&union, a);
            } else {
                let pa = Self::find_parent(&union, a);
                let pb = Self::find_parent(&union, b);
                if pa == pb {
                    return edge;
                }
                union[b] = pa;
                union[pb] = pa;
            }        
        }
        return Vec::new();
    }
    fn find_parent(u: &Vec<usize>, p: usize) -> usize {
        if u[p] == p || u[p] == 0 {
            return u[p];
        }
        return Self::find_parent(u, u[p]);
    }
}
