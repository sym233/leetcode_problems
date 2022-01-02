impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let rows: Vec<i32> = bank
            .into_iter()
            .map(|s| s.chars().filter(|&c| c == '1').count() as i32)
            .filter(|&n| n > 0)
            .collect();
        return rows
            .windows(2)
            .map(|w| w[0] * w[1])
            .sum();
    }
}
