use std::collections::BinaryHeap;
impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let mut pq = BinaryHeap::new();
        // binary heap is maximum heap
        pq.push(-1);
        let mut prev = 0;
        let mut i = 1;
        return loop {
            let front = pq.pop().unwrap();
            if front == prev {
                continue;
            }
            prev = front;
            if i == n {
                break -front;
            }
            for p in primes.iter() {
                let mul = p * front;
                if mul / p != front {
                    break;
                }
                pq.push(mul);
            }
            i += 1;
        };
    }
}
