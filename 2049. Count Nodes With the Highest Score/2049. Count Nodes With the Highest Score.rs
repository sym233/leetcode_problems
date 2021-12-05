impl Solution {
    pub fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
        #[derive(Clone)]
        struct Node {
            left_index: i32,
            left_count: i32,
            right_index: i32,
            right_count: i32,
        };
        impl Node {
            fn new() -> Self {
                return Node {
                    left_index: -1,
                    left_count: 0,
                    right_index: -1,
                    right_count: 0,
                };
            }
        }
        
        let l = parents.len();
        let mut tree: Vec<Node> = vec![Node::new(); l];
        let mut root = 0usize;
        
        for i in 0..l {
            if parents[i] == -1 {
                root = i as usize;
                continue;
            }
            let pa = parents[i] as usize;
            if tree[pa].left_index == -1 {
                tree[pa].left_index = i as i32;
            } else {
                tree[pa].right_index = i as i32;
            }
        }
        
        fn dfs(mut tree: &mut Vec<Node>, index: usize) -> i32 {
            let left = tree[index].left_index;
            if left != -1 {
                tree[index].left_count = dfs(&mut tree, left as usize);
            }
            let right = tree[index].right_index;
            if right != -1 {
                tree[index].right_count = dfs(&mut tree, right as usize);
            }
            return tree[index].left_count + tree[index].right_count + 1;
        }
        dfs(&mut tree, root);

        
        let mut score: i64 = 0;
        let mut num = 0;
        
        for node in tree {
            let mut prod = 1;
            let left = node.left_count;
            if left != 0 {
                prod *= left as i64;
            }
            let right = node.right_count;
            if right != 0 {
                prod *= right as i64;
            }
            let pa = l as i32 - left - right - 1;
            if pa != 0 {
                prod *= pa as i64;
            }
            if prod == score {
                num += 1;
            }
            if prod > score {
                score = prod;
                num = 1;
            }
        }
        return num;
    }
}
