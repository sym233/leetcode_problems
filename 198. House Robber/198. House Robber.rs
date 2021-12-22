impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        let mut money_by: Vec<i32> = vec![nums[0]];
        
        for i in 1..l {
            if i == 1 {
                money_by.push(nums[i].max(money_by[i - 1]));
            } else {
                money_by.push((money_by[i - 2] + nums[i]).max(money_by[i - 1]));
            }
        }
        return money_by[l - 1];
    }
}
