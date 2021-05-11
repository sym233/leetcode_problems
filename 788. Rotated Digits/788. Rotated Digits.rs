impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        return (1..=n)
            .map(|i| Self::rotate_num(i).map(|x| x != i).unwrap_or(false))
            .filter(|b| *b)
            .count() as i32;
    }
    fn rotate_digit(n: i32) -> Option<i32> {
        return match n {
            0 => Some(0),
            1 => Some(1),
            2 => Some(5),
            5 => Some(2),
            6 => Some(9),
            8 => Some(8),
            9 => Some(6),
            _ => None,
        };
    }
    fn rotate_num(n: i32) -> Option<i32> {
        let rightmost = n % 10;
        if let Some(x) = Self::rotate_digit(rightmost){
            if n / 10 > 0 {
                if let Some(y) = Self::rotate_num(n / 10) {
                    return Some(y * 10 + x);
                }
                return None;
            }
            return Some(x);            
        }
        return None;
    }
}
