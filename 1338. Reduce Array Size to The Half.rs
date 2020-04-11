use std::collections::HashMap;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut h: HashMap<i32, usize> = HashMap::new();
        
        struct Item {
            num: i32,
            count: usize,
        }
        
        let mut v: Vec<Item> = Vec::new();
        let l = arr.len();
        for n in arr {
            if let Some(&i) = h.get(&n) {
                v[i].count += 1;
            } else {
                h.insert(n, v.len());
                v.push(Item {
                    num: n,
                    count: 1,
                });
            }
        }
        v.sort_by(|item1, item2| item2.count.cmp(&item1.count));
        let mut count = 0usize;
        let mut del_count = 0;
        for item in v.iter() {
            count += item.count;
            del_count += 1;
            if count >= l / 2 {
                break;
            }
        }
        return del_count;
    }
}
