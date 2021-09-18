impl Solution {
    pub fn calculate(s: String) -> i32 {
        enum Item {
            Plus,
            Minus,
            LeftPar,
            RightPar,
            Num(i32),
        }
        fn clear_stack(mut stack: &mut Vec<Item>) {
            let len = stack.len();
            if len == 2 {
                match (&stack[0], &stack[1]) {
                    (&Item::Minus, &Item::Num(n)) => {
                        stack.truncate(0);
                        stack.push(Item::Num(-n));
                    },
                    _ => (),
                }
            }
            if len < 3 {
                return;
            }
            let offset = stack.len() - 3;
            match (&stack[offset], &stack[offset + 1], &stack[offset + 2]) {
                (&Item::Num(n1), &Item::Plus, &Item::Num(n2)) => {
                    stack.truncate(offset);
                    stack.push(Item::Num(n1 + n2));
                    clear_stack(&mut stack);
                },
                (&Item::Num(n1), &Item::Minus, &Item::Num(n2)) => {
                    stack.truncate(offset);
                    stack.push(Item::Num(n1 - n2));
                    clear_stack(&mut stack);
                },
                (&Item::LeftPar, &Item::Num(n), &Item::RightPar) => {
                    stack.truncate(offset);
                    stack.push(Item::Num(n));
                    clear_stack(&mut stack);
                },
                (&Item::LeftPar, &Item::Minus, &Item::Num(n)) => {
                    stack.truncate(offset + 1);
                    stack.push(Item::Num(-n));
                },
                _ => (),
            }
        }
        fn to_num(s: &str) -> Item {
            return Item::Num(s.parse().unwrap());
        }
        
        let mut temp = String::new();
        let mut stack: Vec<Item> = vec![];
        
        for c in s.chars() {
            if c.is_digit(10) {
                temp.push(c);
            } else {
                if !temp.is_empty() {
                    stack.push(to_num(&temp));
                    temp = String::new();
                }
                clear_stack(&mut stack);
                
                match c {
                    '+' => stack.push(Item::Plus),
                    '-' => stack.push(Item::Minus),
                    '(' => stack.push(Item::LeftPar),
                    ')' => stack.push(Item::RightPar),
                    _ => (),
                }
            }
        }
        if !temp.is_empty() {
            stack.push(to_num(&temp));
        }
        clear_stack(&mut stack);
        if let Item::Num(n) = stack[0] {
            return n;
        }
        panic!();
    }
}
