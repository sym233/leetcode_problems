use std::collections::VecDeque;
use std::ops::{Add, Div, Mul, Sub};

impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        let target = 24;

        #[derive(Clone, Copy)]
        struct Frac {
            num: i32,
            den: i32,
        };

        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }

        impl Frac {
            fn reduce(&mut self) {
                if self.num == 0 {
                    self.den = 1;
                    return;
                }
                if self.den < 0 {
                    self.num *= -1;
                    self.den *= -1;
                }
                let d = gcd(self.num.abs(), self.den.abs());
                self.num /= d;
                self.den /= d;
            }
        }

        impl From<i32> for Frac {
            fn from(num: i32) -> Frac {
                Frac { num, den: 1 }
            }
        }

        impl Add for Frac {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                let mut res = Frac {
                    num: self.num * rhs.den + rhs.num * self.den,
                    den: self.den * rhs.den,
                };
                res.reduce();
                res
            }
        }

        impl Sub for Frac {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                let mut res = Frac {
                    num: self.num * rhs.den - rhs.num * self.den,
                    den: self.den * rhs.den,
                };
                res.reduce();
                res
            }
        }

        impl Mul for Frac {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                let mut res = Frac {
                    num: self.num * rhs.num,
                    den: self.den * rhs.den,
                };
                res.reduce();
                res
            }
        }

        impl Div for Frac {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                let mut res = Frac {
                    num: self.num * rhs.den,
                    den: self.den * rhs.num,
                };
                res.reduce();
                res
            }
        }

        let can = cards.into_iter().map(Frac::from).collect::<Vec<_>>();
        let mut q = VecDeque::from([can]);

        while let Some(can) = q.pop_front() {
            let len = can.len();
            if len == 1 && can[0].num == target && can[0].den == 1 {
                return true;
            }

            for i in 0..len {
                for j in i + 1..len {
                    let mut new_can = vec![];
                    for k in 0..len {
                        if k == i || k == j {
                            continue;
                        }
                        new_can.push(can[k]);
                    }

                    let lhs = can[i];
                    let rhs = can[j];

                    {
                        let mut new_can_clone = new_can.clone();
                        new_can_clone.push(lhs + rhs);
                        q.push_back(new_can_clone);
                    }
                    {
                        let mut new_can_clone = new_can.clone();
                        new_can_clone.push(lhs - rhs);
                        q.push_back(new_can_clone);
                    }
                    {
                        let mut new_can_clone = new_can.clone();
                        new_can_clone.push(rhs - lhs);
                        q.push_back(new_can_clone);
                    }
                    {
                        let mut new_can_clone = new_can.clone();
                        new_can_clone.push(lhs * rhs);
                        q.push_back(new_can_clone);
                    }
                    if rhs.num != 0 {
                        let mut new_can_clone = new_can.clone();
                        new_can_clone.push(lhs / rhs);
                        q.push_back(new_can_clone);
                    }
                    if lhs.num != 0 {
                        let mut new_can_clone = new_can.clone();
                        new_can_clone.push(rhs / lhs);
                        q.push_back(new_can_clone);
                    }
                }
            }
        }

        false
    }
}
