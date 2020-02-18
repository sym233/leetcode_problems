use std::collections::HashMap;
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, d: i32) -> i32 {
        let mut maxW = 0;
        let mut totalW = 0;
        
        for &n in weights.iter() {
            if n > maxW {
                maxW = n;
            }
            totalW += n;
        }
        
        let mut cache = HashMap::new();
                
        let mut calc_days = |w: i32| -> i32 {
            if let Some(v) = cache.get(&w) {
                return *v;
            }
            
            let mut days = 0;
            let mut load = 0;
            
            for &n in weights.iter() {
                if load + n > w {
                    load = 0;
                    days += 1;
                }
                load += n;
            }
            if load > 0 {
                load = 0;
                days += 1;
            }
            cache.insert(w, days);
            return days;
        };
        
        let mut s = maxW;
        let mut e = totalW + 1;
        
        while e - s > 1 {
            let m = (e + s) / 2;
            let days = calc_days(m);
            if days <= d {
                e = m;
            } else {
                s = m;
            }
        }
        
        let days = calc_days(s);
        return if days > d {
            e
        } else {
            s
        };
    }
}
