use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        // From<[T; N]> is introduced in ver 1.56.0
        // lc uses 1.45.2, which only support From<Vec<T>>
        // pop for BTreeSet is still nightly
        let mut q: BinaryHeap<Reverse<i32>> = BinaryHeap::from(vec![Reverse(1)]);
        let factors = [2, 3, 5];
        let mut i = 1;
        let mut prev = 0;
        while let Some(Reverse(v)) = q.pop() {
            if v == prev {
                continue;
            }
            prev = v;
            if i == n {
                return v;
            }
            i += 1;
            for &f in &factors {
                let p = v * f;
                if v == p / f {
                    q.push(Reverse(p));
                } else {
                    // ovreflow
                    // println!("{} * {} = {}", v, f, p);
                }
            }
        }
        panic!();
    }
}
