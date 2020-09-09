impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        if k % 2 == 1 {
            return Self::kth_grammar(n - 1, k / 2 + 1);
        } else {
            return Self::not(Self::kth_grammar(n - 1, k / 2));
        }
        
    }
    fn not(n: i32) -> i32 {
        if n == 0 {
            return 1;
        } else {
            return 0;
        }
    }
}
