impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let mut a = 0;
        let mut b = 0;
        let mut aseq = 0;
        let mut bseq = 0;
        for c in colors.chars() {
            if c == 'A' {
                aseq += 1;
                bseq = 0;
                if aseq >= 3 {
                    a += 1;
                }
            } else {
                bseq += 1;
                aseq = 0;
                if bseq >= 3 {
                    b += 1;
                }
            }
        }
        return a > b;
    }
}
