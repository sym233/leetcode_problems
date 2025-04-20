use std::collections::HashSet;
impl Solution {
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        let n = n as usize;
        let k = k as i64;
        let mut s = HashSet::<[i8; 10]>::new();
        let mut res = 0i64;

        fn form_n_digit_pal(n: usize) -> Vec<Vec<i8>> {
            let mut res: Vec<Vec<i8>> = vec![];
            let mut digits = vec![0; n / 2 + n % 2];

            digits[0] = 1;
            let is_odd_digit = n % 2 == 1;
            
            let mut num = digits.clone();
            if is_odd_digit {
                num.pop();
            }
            num.extend(digits.iter().rev().map(|x| *x).collect::<Vec<i8>>());
            res.push(num);
            
            fn dfs(res: &mut Vec<Vec<i8>>, digits: &mut Vec<i8>, is_odd_digit: bool) {
                let l = digits.len();
                let mut i = l - 1;
                while digits[i] == 9 {
                    if i == 0 {
                        return;
                    }
                    digits[i] = 0;
                    i -= 1;
                }
                digits[i] += 1;

                let mut num = digits.clone();
                if is_odd_digit {
                    num.pop();
                }
                num.extend(digits.iter().rev().map(|x| *x).collect::<Vec<i8>>());

                res.push(num);
                dfs(res, digits, is_odd_digit);
            }
            dfs(&mut res, &mut digits, is_odd_digit);
            res
        }
        fn vec_digit_to_num(digits: &Vec<i8>) -> i64 {
            let mut num = 0i64;
            for &digit in digits.iter() {
                num *= 10;
                num += digit as i64;
            }
            num
        }
        fn count_rearrange(num_count: &[i8; 10]) -> i64 {
            let fact = |x| [1i64, 1, 2, 6, 24, 120, 720, 5_040, 40_320, 362_880, 3_628_800][x as usize];
            let nums = num_count.iter().sum::<i8>() as i64;
            let zero_num = num_count[0] as i64;
            let mut rearrange_num = fact(nums);
            for i in 0..=9 {
                rearrange_num /= fact(num_count[i] as i64);
            }
            if zero_num > 0 {
                let mut invalid_num = fact(nums - 1);
                invalid_num /= fact(zero_num - 1);
                for i in 1..=9 {
                    invalid_num /= fact(num_count[i] as i64);
                }
                rearrange_num -= invalid_num;
            }
            rearrange_num
        }

        form_n_digit_pal(n)
            .into_iter()
            .filter(|x| vec_digit_to_num(x) % k == 0)
            .map(|x| {
                let mut num_count = [0; 10];
                x.into_iter().for_each(|digit| num_count[digit as usize] += 1);
                num_count
            })
            .for_each(|x| {
                s.insert(x);
            });
        s.into_iter().map(|x| count_rearrange(&x)).sum()
    }
}
