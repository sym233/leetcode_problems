const m: i64 = 1_000_000_007;

impl Solution {
    pub fn count_texts(pressed_keys: String) -> i32 {        
        let mut res = 1;
        let keys = pressed_keys.as_bytes();
        let l = keys.len();
        let (p3, p4) = Self::posibles(l);
        let mut len = 1;
        for i in 1..=l {
            if i < l && keys[i - 1] == keys[i] {
                len += 1;
            } else {
                match keys[i - 1] {
                    b'7' | b'9' => {
                        res *= p4[len];
                    }
                    _ => {
                        res *= p3[len];
                    }
                }
                res %= m;
                len = 1;
            }
        }
        
        res as i32
    }
    fn posibles(max: usize) -> (Vec<i64>, Vec<i64>) {
        let mut p3 = vec![0; max + 1];
        for i in 1..=max {
            for j in 1..=3 {
                if i > j {
                    p3[i] += p3[i - j];
                } else if i == j {
                    p3[i] += 1;
                }
                p3[i] %= m;
            }
        }
        let mut p4 = vec![0; max + 1];
        for i in 1..=max {
            for j in 1..=4 {
                if i > j {
                    p4[i] += p4[i - j];
                } else if i == j{
                    p4[i] += 1;
                }
                p4[i] %= m;
            }
        }
        (p3, p4)
    }
}
