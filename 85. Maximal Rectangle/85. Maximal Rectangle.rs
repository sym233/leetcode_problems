impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut left_acc: Vec<Vec<i32>> = vec![vec![0; n]; m];
        let mut max = 0;
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == '1' {
                    if j > 0 && matrix[i][j - 1] == '1' {
                        left_acc[i][j] = left_acc[i][j - 1] + 1;
                    } else {
                        left_acc[i][j] = 1;
                    }
                }
            }
        }
        for j in 0..n {
            for i in 0..m {
                let mut min = i32::MAX;
                for i2 in i..m {
                    if left_acc[i2][j] > 0 {
                        min = min.min(left_acc[i2][j]);
                        max = max.max((i2 - i + 1) as i32 * min);
                    } else {
                        break;
                    }
                }
            }
        }
        return max;
    }
}
