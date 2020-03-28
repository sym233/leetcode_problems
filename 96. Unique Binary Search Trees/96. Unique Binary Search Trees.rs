impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut table = vec![0; n + 1];
        table[0] = 1;
        
        for total_number_of_nodes in 1..=n {
            for left_tree_nodes in 0..total_number_of_nodes {
                table[total_number_of_nodes] += 
                    table[left_tree_nodes] * table[total_number_of_nodes - left_tree_nodes - 1];
            }
        }
        return table[n];
    }
}