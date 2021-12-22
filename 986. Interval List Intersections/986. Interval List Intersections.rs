impl Solution {
    pub fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let l1 = first_list.len();
        let l2 = second_list.len();
        let mut i1 = 0;
        let mut i2 = 0;
        let mut res: Vec<Vec<i32>> = Vec::new();
        while i1 < l1 && i2 < l2 {
            let c1 = &first_list[i1];
            let c2 = &second_list[i2];
            if c1[1] < c2[0] {
                i1 += 1;
            } else if c2[1] < c1[0] {
                i2 += 1;
            } else {
                let s = c1[0].max(c2[0]);
                let e = c1[1].min(c2[1]);
                let mut updated = false;
                if let Some(c) = res.last_mut() {
                    if c[1] == s {
                        c[1] = e;
                        updated = true;
                    }
                }
                if !updated {
                    let c = vec![s, e];
                    res.push(c);
                }
                if c1[1] <= c2[1] {
                    i1 += 1;
                } else {
                    i2 += 1;
                }
            }
        }
        return res;
    }
}
