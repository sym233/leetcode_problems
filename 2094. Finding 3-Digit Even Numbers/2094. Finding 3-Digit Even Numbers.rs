use std::collections::BTreeSet;
impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut res: BTreeSet<i32> = BTreeSet::new();
        let l = digits.len();
        for i in 0..l {
            if digits[i] == 0 {
                continue;
            }
            for j in 0..l {
                if i == j {
                    continue;
                }
                for k in 0..l {
                    if k != i && k != j && digits[k] % 2 == 0 {
                        res.insert(digits[i] * 100 + digits[j] * 10 + digits[k]);
                    }
                }
            }
        }
        return res.into_iter().collect();
    }
}
