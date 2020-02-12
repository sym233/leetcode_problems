impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        const OFFSET: i32 = 1000;
        let mut numberCount = [0 as usize; 2001];
        let mut occurrenceCount = [0 as usize; 1000];
        for num in arr.iter() {
            numberCount[(num + OFFSET) as usize] += 1;
        }
        for occu in numberCount.iter() {
            occurrenceCount[*occu] += 1;
        }
        for oc in occurrenceCount[1..].iter() {
            if *oc > 1 {
                return false;
            }
        }
        return true;
    }
}
