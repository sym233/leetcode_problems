impl Solution {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        const L: usize = 26;
        const A: usize = 'a' as usize;
        const M: usize = 100;
        type Table = [usize; L];
        fn strToArr(s: &str) -> Table {
            let mut arr: Table = [0; L];
            for c in s.chars() {
                arr[c as usize - A] += 1;
            }
            return arr;
        }
        
        fn union(arr1: Table, arr2: Table) -> Table {
            let mut res: Table = [0; L];
            for i in 0..L {
                res[i] = if arr1[i] < arr2[i] {
                    arr1[i]
                } else {
                    arr2[i]
                };
            }
            return res;
        }
        
        let mut table: Table = [M; L];
        for s in a.iter() {
            table = union(table, strToArr(&s));
        }
        let mut res: Vec<String> = vec!();
        for i in 0..L {
            for j in 0..table[i] {
                res.push((((i + A) as u8) as char).to_string());
            }
        }
        return res;
    }
}
