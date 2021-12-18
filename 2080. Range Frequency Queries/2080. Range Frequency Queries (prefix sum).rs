use std::collections::HashMap;
struct RangeFreqQuery {
    maps: Vec<HashMap<i32, i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeFreqQuery {

    fn new(arr: Vec<i32>) -> Self {
        let mut maps: Vec<HashMap<i32, i32>> = Vec::new();
        maps.push(HashMap::new());
        for n in arr {
            let mut last = maps.last().unwrap().clone();
            if let Some(count) = last.get_mut(&n) {
                *count += 1;
            } else {
                last.insert(n, 1);
            }
            maps.push(last);
        }
        return Self { maps };
    }
    
    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        let l = if let Some(v) = self.maps[left as usize].get(&value) {
            *v
        } else {
            0
        };
        let r = if let Some(v) = self.maps[right as usize + 1].get(&value) {
            *v
        } else {
            0
        };
        return r - l;
    }
}

/**
 * Your RangeFreqQuery object will be instantiated and called as such:
 * let obj = RangeFreqQuery::new(arr);
 * let ret_1: i32 = obj.query(left, right, value);
 */
