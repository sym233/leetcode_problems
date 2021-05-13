struct NumMatrix {
    pre_sum: Vec<Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {

    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut pre_sum = Vec::<Vec<i32>>::new();
        let R = matrix.len();
        let C = matrix[0].len();
        pre_sum.push(vec![0; C + 1]);
        for r in 0..R {
            pre_sum.push(vec![0]);
            for c in 0..C {
                let n = matrix[r][c] + pre_sum[r][c + 1] + pre_sum[r + 1][c] - pre_sum[r][c];
                pre_sum[r + 1].push(n);
            }
        }
        // println!("{:?}", pre_sum);
        return NumMatrix { pre_sum };
    }
    
    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let c1 = col1 as usize;
        let r1 = row1 as usize;
        let c2 = col2 as usize + 1;
        let r2 = row2 as usize + 1;
        return self.pre_sum[r2][c2]
            - self.pre_sum[r2][c1]
            - self.pre_sum[r1][c2]
            + self.pre_sum[r1][c1] as i32;
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */
