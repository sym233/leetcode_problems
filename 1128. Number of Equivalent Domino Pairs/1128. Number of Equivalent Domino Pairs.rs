impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        const MAXD: usize = 10;
        let mut table = [0; MAXD * MAXD];
        
        for domino in dominoes.iter() {
            let value = if (domino[0] <= domino[1]) {
                domino[0] * MAXD as i32 + domino[1]
            } else {
                domino[0] + domino[1] * MAXD as i32
            } as usize;
            count += table[value];
            table[value] += 1;
        }
        return count;
    }
}
