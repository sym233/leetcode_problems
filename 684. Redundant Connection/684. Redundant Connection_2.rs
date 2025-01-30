impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut uf = vec![0; n + 1];

        // i-th node
        for i in 1..=n {
            uf[i] = i;
        }

        for edge in edges {
            let mut a = edge[0] as usize;
            let mut b = edge[1] as usize;

            if uf[a] != uf[b] {
                // a & b not connected
                if uf[b] > uf[a] {
                    (a, b) = (b, a);
                }
                
                let mut p = uf[b];
                while p != uf[p] {
                    p = uf[p];
                }

                uf[p] = uf[a];

                // tight the tree
                for i in 1..=n {
                    if uf[i] == p {
                        uf[i] = uf[p];
                    }
                }               


            } else {
                // connected
                return edge;
            }
        }

        panic!();
    }
}
