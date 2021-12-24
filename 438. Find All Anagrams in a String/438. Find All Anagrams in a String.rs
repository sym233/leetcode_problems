impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let cs: Vec<char> = s.chars().collect();
        let mut pletters: Vec<i32> = vec![0; 26];
        let mut letters: Vec<i32> = vec![0; 26];
        for c in p.chars() {
            pletters[ord(c)] += 1;
        }
        let l = cs.len();
        let pl = p.chars().count();
        let mut i = 0;
        let mut count = 0;
        let mut res: Vec<i32> = Vec::new();
        for c in s.chars() {
            letters[ord(c)] += 1;
            count += 1;
            if count > pl {
                letters[ord(cs[i])] -= 1;
                i += 1;
                count -= 1;
            }
            if count == pl {
                if letters == pletters {
                    res.push(i as i32);
                }
            }
        }
        return res;
    }
}

fn ord(c: char) -> usize {
    return c as u8 as usize - 'a' as u8 as usize;
}
