impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1 == str2 {
            return str1;
        }
        if str1.len() < str2.len() {
            return Self::gcd_of_strings(str2, str1);
        }
        if let Some(rem) = str1.strip_prefix(&str2) {
            return Self::gcd_of_strings(rem.to_string(), str2);
        }
        return String::new();
    }
}
