impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        // union find
        let mut uf = [0; 1001];
        
        for edge in &edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            if uf[a] == 0 {
                if uf[b] == 0 {
                    uf[a] = a;
                    uf[b] = a;
                } else {
                    let mut i = uf[b];
                    while uf[i] != i {
                        i = uf[i];   
                    }
                    uf[a] = i;
                }
            } else {
                if uf[b] == 0 {
                    let mut i = uf[a];
                    while uf[i] != i {
                        i = uf[i];   
                    }
                    uf[b] = i;
                } else {
                    // uf[a] != 0 && uf[b] != 0
                    let mut i = uf[a];
                    while uf[i] != i {
                        i = uf[i];   
                    }
                    let mut j = uf[b];
                    while uf[j] != j {
                        j = uf[j];   
                    }
                    
                    if i != j {
                        uf[i] = j;
                    } else {
                        return vec![a as i32, b as i32];
                    }
                } 
            }
        }
        
        return Vec::new();
    }
}
