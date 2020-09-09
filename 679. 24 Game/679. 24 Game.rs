impl Solution {
    pub fn judge_point24(nums: Vec<i32>) -> bool {
	    // convert the numbers into float for the convinience of division
        let nums: Vec<f64> = nums.iter().map(|&x| x as f64).collect();
        
		// define 6 operators, be careful for division by 0
        fn add(a: f64, b: f64) -> Option<f64> {
            return Some(a + b);
        }
        fn sub(a: f64, b: f64) -> Option<f64> {
            return Some(a - b);
        }
        fn subr(a: f64, b: f64) -> Option<f64> {
            return Some(b - a);
        }
        fn mul(a: f64, b: f64) -> Option<f64> {
            return Some(a * b);
        }
        fn div(a: f64, b: f64) -> Option<f64> {
            if b != 0.0 {
                return Some(a / b);
            } else {
                return None;
            }
        }
        fn divr(a: f64, b: f64) -> Option<f64> {
            if a != 0.0 {
                return Some(b / a);
            } else {
                return None;
            }
        }
        
		// store opterators in an array for convinient access
        const F: [fn(f64, f64) -> Option<f64>; 6] = [
            add, sub, subr, mul, div, divr
        ];
        
        // search for all possible values based on current numbers
        fn calc(nums: Vec<f64>) -> Vec<Vec<f64>> {
            let l = nums.len();
			// if nums has only 1 element, it could not be operated any more
            if l <= 1 {
                return vec![nums];
            }
            let mut res: Vec<Vec<f64>> = Vec::new();
            
            for i in 0..(l - 1) {
                for j in (i + 1)..l {
				    // choose 2 numbers from nums as operands
                    let n1 = nums[i];
                    let n2 = nums[j];
					// remove 2 chosen numbers from nums
                    let nums: Vec<f64> = nums
                        .iter()
                        .enumerate()
                        .filter(|p| p.0 != i && p.0 != j)
                        .map(|p| *p.1)
                        .collect();
                    
					// perform each operator on operands and push the results back to nums, respectively
					// then the length of nums reduces by 1
                    for f in F.iter() {
                        let mut nums = nums.clone();
                        if let Some(n) = f(n1, n2) {
                            nums.push(n);
                            res.push(nums);
                        }
                    }
                }
            }
            return res;
        }
        
		// stack and initial nums
        let mut stack: Vec<Vec<f64>> = Vec::new();
        stack.push(nums);
        
		// when the stack is not empty, search for new numbers based on top 
        while let Some(top) = stack.pop() {
		    // find 24, be careful for the float number comparision
            if top.len() == 1 && (top[0] - 24.0).abs() < 0.01 {
                return true;
            }
            if top.len() > 1 {
                stack.append(&mut calc(top));
            }
        }
        
		// solution not found
        return false;
    }
}
