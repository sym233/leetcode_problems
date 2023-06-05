impl Solution {
  pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut new_interval = Some(new_interval);
    for interval in intervals {
      if let Some(ref mut ni) = new_interval {
        if ni[1] < interval[0] {
          res.push(new_interval.take().unwrap());
          res.push(interval);
        } else if interval[1] < ni[0] {
          res.push(interval);
        } else {
          ni[0] = ni[0].min(interval[0]);
          ni[1] = ni[1].max(interval[1]);
        }
      } else {
        res.push(interval);
      }
    }
    if let Some(ni) = new_interval {
      res.push(ni);
    }
    res
  }
}
