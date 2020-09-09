impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let target = target as usize;
        
        let mut res: Vec<Vec<Vec<i32>>> = vec![vec![]; target + 1];
        
        for i in 1..=target {
            for &j in candidates.iter() {
                let ju = j as usize;
                if ju == i {
                    res[i].push(vec![j]);
                } else if i > ju && !res[i - ju].is_empty() {
                    let mut to_append: Vec<Vec<i32>> = Vec::new();
                    for ar in res[i - ju].iter() {
                        if *ar.last().unwrap() <= j {
                            let mut new_ar = ar.clone();
                            new_ar.push(j);
                            to_append.push(new_ar);
                        }
                    }
                    res[i].append(&mut to_append);
                }
            }
        }
        return res.swap_remove(target);
    }
}
