impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let l = cost.len();
        let mut arrival: Vec<i32> = vec![0, 0];
        for i in 2..=l {
            arrival.push((arrival[i - 1] + cost[i - 1]).min(arrival[i - 2] + cost[i - 2]));
        }
        return arrival[l];
    }
}
