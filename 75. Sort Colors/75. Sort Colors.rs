impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut bucket = [0, 0, 0];
        for n in nums.iter() {
            bucket[*n as usize] += 1;
        }
        let mut ni = nums.iter_mut();
        for (i, count) in bucket.into_iter().enumerate() {
            for _ in 0..count {
                *ni.next().unwrap() = i as i32;
            }
        }
    }
}
