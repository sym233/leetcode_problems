use std::iter;
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        //      a  e  i  o  u    
        //         
        // a    0  1  0  0  0         1          1  
        // e    1  0  1  0  0         1          2
        // i  [ 1  1  0  1  1 ]  *  [ 1 ]  ->  [ 4 ] 
        // o    0  0  1  0  1         1          2
        // u    1  0  0  0  0         1          1
                        
        // total:                     5          10
        
        // thererfore, res = sum(A ^ (n - 1) * ones(5, 1))
        
        const L: usize = 5usize;
        const M: i32 = 1e9 as i32 + 7;
        let mat: Vec<Vec<i32>> = vec![
            vec![0, 1, 0, 0, 0],
            vec![1, 0, 1, 0, 0],
            vec![1, 1, 0, 1, 1],
            vec![0, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 0],
        ];
        
        fn mm(a: i32, b: i32, m: i32) -> i32 {
            let a = a as i64;
            let b = b as i64;
            let m = m as i64;
            return ((a * b) % m) as i32;
        }   
        fn mul(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            let ra = a.len();
            let ca = a[0].len();
            let rb = b.len();
            let cb = b[0].len();
            assert_eq!(ca, rb);
            
            let mut res = vec![vec![0; cb]; ra];
            for r in 0..ra {
                for c in 0..cb {
                    for i in 0..ca {
                        res[r][c] += mm(a[r][i], b[i][c], M);
                        res[r][c] %= M;
                    }
                }
            }
            return res;
        }
        fn pow(a: &Vec<Vec<i32>>, n: usize) -> Vec<Vec<i32>> {
            let r = a.len();
            assert_eq!(r, a[0].len());
            if n == 0 {
                let mut res = vec![vec![0; r]; r];
                for i in 0..r {
                    res[i][i] = 1;
                }
                return res;
            } else if n == 1 {
                return a.clone();
            } else if n % 2 == 0 {
                return pow(&mul(a, a), n / 2);
            } else {
                return mul(&pow(a, n - 1), a);
            }
        }
        
        let v = vec![
            vec![1],
            vec![1],
            vec![1],
            vec![1],
            vec![1],
        ];
        let mat = pow(&mat, (n - 1) as usize);
        let v = mul(&mat, &v);
        return v.iter().fold(0, |acc, item| (acc + item[0]) % M);
    }
}
