impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let size = seats.len() as i32;
        let occuppied_seats: Vec<i32> = seats
            .iter()
            .enumerate()
            .filter(|(_, occuppied)| **occuppied == 1)
            .map(|(i, _)| i as i32)
            .collect();
        let mut distances: Vec<i32> = occuppied_seats
            .windows(2)
            .map(|window| (window[1] - window[0]) / 2)
            .collect();
        distances.push(occuppied_seats[0]);
        distances.push(size - *occuppied_seats.last().unwrap() - 1);
        // println!("{:?}", distances);
        return *distances.iter().max().unwrap();
    }
}
