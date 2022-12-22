use std::ops::{ Add, Sub, Mul, Div };
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for token in tokens {
            let mut operator: Option<fn(i32, i32) -> i32> = None;
            match token.as_str() {
                "+" => operator = Some(i32::add),
                "-" => operator = Some(i32::sub),
                "*" => operator = Some(i32::mul),
                "/" => operator = Some(i32::div),
                num => stack.push(num.parse::<i32>().unwrap()),
            }
            if let Some(op) = operator {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(op(lhs, rhs));
            }
        }
        return stack.pop().unwrap();
    }
}
