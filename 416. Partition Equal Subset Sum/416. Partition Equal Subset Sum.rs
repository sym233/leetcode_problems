impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut sum = 0;
        for &n in nums.iter() {
            sum += n;
        }
        if sum % 2 != 0 {
            return false;
        }
        let sum = sum / 2;
        
        const MAXS: usize = 10001usize;
        let mut table = [false; MAXS];
        table[0] = true;
        for &n in nums.iter() {
            for s in (0..=sum).rev() {
                if table[s as usize] {
                    let currSum = n + s;
					if currSum == sum {
					    return true;
				    }
                    table[currSum as usize] = true;
                }
            }
        }
        return false;
    }
}
