impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        
        let mut s: HashSet<i32> = HashSet::new();
        for n in nums {
            if !s.insert(n) {
                return true;
            }
        }
        return false;
    }
}
