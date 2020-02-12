impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let num_people = num_people as usize;
        let mut candies = candies as usize;
        let mut dist: Vec<i32> = vec![0; num_people];
        for i in (1 as usize).. {
            if i < candies {
                dist[(i - 1) % num_people] += i as i32;
                candies -= i;
            } else {
                dist[(i - 1) % num_people] += candies as i32;
                break;
            }
        }
        return dist;
    }
}
