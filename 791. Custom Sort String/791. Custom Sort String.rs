use std::collections::HashMap;
use std::iter::FromIterator;
use std::str::from_utf8;

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut m = HashMap::<u8, usize>::from_iter(order.bytes().enumerate().map(|(i, b)| (b, i)));
        let mut rest = vec![];
        let mut to_sort = vec![];
        for b in s.bytes() {
            if let Some(&v) = m.get(&b) {
                to_sort.push(v);
            } else {
                rest.push(b);
            }
        }
        to_sort.sort();
        let mut res = String::from_utf8(to_sort.into_iter().map(|v| order.as_bytes()[v]).collect()).unwrap();
        res += from_utf8(&rest).unwrap();
        return res;
    }
}
