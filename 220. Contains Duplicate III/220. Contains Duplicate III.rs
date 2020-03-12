impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        if t < 0 {
            // damn test case
            return false;
        }
        
        use std::collections::BTreeSet;
        let mut s: BTreeSet<i64> = BTreeSet::new();
        
        let l = nums.len();
        let k = k as usize;
        let t = t as i64;
        
        for i in 0..l {
            if i > k {
                let j = i - k - 1;
                s.remove(&(nums[j] as i64));
            }
            let v = nums[i] as i64;
            let mut r = s.range((v - t)..=(v + t));
            if r.next() != None {
                // r is not an empty iter
                return true;
            }
            s.insert(v);
        }        
        return false;
    }
}
