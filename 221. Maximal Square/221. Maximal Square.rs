impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let h = matrix.len();
        if h == 0 {
            return 0;
        }
        let w = matrix[0].len();
        if w == 0 {
            return 0;
        }
        
        // continuous 1's in left and up
        let mut seq: Vec<Vec<(i32, i32)>> = vec![vec![(0, 0); w]; h];
        let mut dp: Vec<Vec<i32>> = vec![vec![0; w]; h];
        let mut m = 0;
        
        for hi in 0..h {
            for wi in 0..w {
                if matrix[hi][wi] == '1' {
                    seq[hi][wi].0 = if hi == 0 {
                        1
                    } else {
                        seq[hi - 1][wi].0 + 1
                    };
                    seq[hi][wi].1 = if wi == 0 {
                        1
                    } else {
                        seq[hi][wi - 1].1 + 1
                    };
                    
                    let mins = [
                        seq[hi][wi].0,
                        seq[hi][wi].1,                    
                        if hi > 0 && wi > 0 {
                            dp[hi - 1][wi - 1] + 1
                        } else {
                            1
                        }
                    ];
                    
                    dp[hi][wi] = *mins.iter().min().unwrap();
                    
                    if dp[hi][wi] > m {
                        m = dp[hi][wi];
                    }
                }
            }
        }
        
        return m * m;
    }
}
