impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        use std::collections::HashMap;
        
        const L: usize = 10usize;
        const MASK: usize = 0xFFFFFusize;
        
        fn nucleotides_to_num(c: char) -> usize {
            return match c {
                'A' => 0,
                'C' => 1,
                'G' => 2,
                'T' => 3,
                _ => {
                    panic!("unknown nucleotides");
                }
            };
        }
        
        let mut res: Vec<String> = Vec::new();
        let mut m: HashMap<usize, bool> = HashMap::new();
        let mut v = 0usize;
        
        for (c, i) in s.chars().zip(0..s.len()) {
            v <<= 2;
            v += nucleotides_to_num(c);
            
            if i >= L - 1 {
                v &= MASK;
                match m.get(&v) {
                    Some(true) => continue,
                    Some(false) => {
                        m.insert(v, true);
                        res.push(String::from(&s[(i - L + 1)..=i]));
                    },
                    None => {
                        m.insert(v, false);
                    },
                }
            }
        }
        
        return res;
    }
}
