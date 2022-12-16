use std::collections::HashMap;
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        fn count_item<T>(m: &mut HashMap<T, usize>, w: &[T])
        where T : std::hash::Hash + Copy + std::cmp::Eq
        {
            for &b in w {
                m.entry(b).and_modify(|counter| *counter += 1).or_insert(1);
            }
        }
        fn contains_all(s: &str, ws: &Vec<String>) -> bool {
            let mut m = HashMap::<&str, usize>::new();
            let l = ws[0].len();
            for i in 0..s.len() / l {
                count_item(&mut m, &[&s[i * l..(i + 1) * l]]);
            }
            for w in ws {
                if let Some(count) = m.get_mut(&w[..]) {
                    *count -= 1;
                } else {
                    return false;
                }
            }
            return m.iter().all(|(_, v)| *v == 0);
        }
        let mut words_len = 0;
        let mut letters = HashMap::new();
        for w in &words {
            words_len += w.len();
            count_item(&mut letters, w.as_bytes());
        }
        let words_len = words_len;
        let letters = letters;
        let b = s.as_bytes();
        let mut in_window = HashMap::new();
        let mut res = vec![];
        for i in 0..s.len() {
            count_item(&mut in_window, &b[i..i + 1]);
            if words_len <= i + 1 {
                if words_len < i + 1 {
                    *in_window.get_mut(&b[i - words_len]).unwrap() -= 1;
                }

                let eq = in_window
                    .iter()
                    .all(|(k, v)| v == &0 || letters.get(k) == Some(v));
                if eq {
                    if contains_all(&s[i - words_len + 1..=i], &words) {
                        res.push((i - words_len + 1) as i32);
                    }
                }   
            }
        }
        return res;
    }
}
