impl Solution {
    pub fn is_number(s: String) -> bool {
        let END = 999usize;
        let fsm: [&Fn(char)->Option<usize>; 8] = [
            &|c| match c {
                '+' | '-' => Some(1),
                '0'..='9' => Some(2),
                '.' => Some(3),
                _ => None,
            }, // 0: begin
            &|c| match c {
                '0'..='9' => Some(2),
                '.' => Some(3),
                _ => None,
            }, // 1: sign
            &|c| match c {
                '0'..='9' => Some(2),
                '.' => Some(4),
                'E' | 'e' => Some(5),
                '\0' => Some(END), // end
                _ => None,
            }, // 2: a integer part digit
            &|c| match c {
                '0'..='9' => Some(4),
                _ => None,
            }, // 3: first demical digit without integer part
            &|c| match c {
                '0'..='9' => Some(4),
                'E' | 'e' => Some(5),
                '\0' => Some(END),
                _ => None, 
            }, // 4: demical part digit
            &|c| match c {
                '+' | '-' => Some(6),
                '0'..='9' => Some(7),
                _ => None,
            }, // 5: e sign
            &|c| match c {
                '0'..='9' => Some(7),
                _ => None,
            }, // 6: e 1st digit
            &|c| match c {
                '0'..='9' => Some(7),
                '\0' => Some(END),
                _ => None,
            }, // 7: e rest digit
        ];
        
        let mut i = 0usize;
        for c in s.chars() {
            if let Some(next_i) = (*fsm[i])(c) {
                i = next_i;
            } else {
                return false;
            }
        }
        return Some(END) == (*fsm[i])('\0');
    }
}
