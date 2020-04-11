use std::cmp::Ordering;
impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        
        fn count_1(&n: &i32) -> usize {
            let mut n = n;
            let mut res = 0;
            while n != 0 {
                res += n & 1;
                n >>= 1;
            }
            return res as usize;
        }
        fn comp(a: &i32, b: &i32) -> Ordering {
            let res = count_1(a).cmp(&count_1(b));
            if res == Ordering::Equal {
                return a.cmp(b);
            } else {
                return res;
            }
        }
        
        arr.sort_by(comp);
        
        return arr;
    }
}
