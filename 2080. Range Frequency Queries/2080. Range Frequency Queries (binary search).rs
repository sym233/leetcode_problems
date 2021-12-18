use std::collections::HashMap;
struct RangeFreqQuery {
    maps: HashMap<i32, Vec<usize>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeFreqQuery {

    fn new(arr: Vec<i32>) -> Self {
        let mut maps: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, n) in arr.into_iter().enumerate() {
            if let Some(vec) = maps.get_mut(&n) {
                vec.push(i);
            } else {
                maps.insert(n, vec![i]);
            }
        }
        return Self { maps };
    }
    
    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        let left = left as usize;
        let right = right as usize + 1;
        if let Some(vec) = self.maps.get(&value) {
            let l = vec.binary_search(&left).unwrap_or_else(|x| x);
            let r = vec.binary_search(&right).unwrap_or_else(|x| x);
            return (r - l) as i32;
        }
        return 0;
    }
}

/**
 * Your RangeFreqQuery object will be instantiated and called as such:
 * let obj = RangeFreqQuery::new(arr);
 * let ret_1: i32 = obj.query(left, right, value);
 */
