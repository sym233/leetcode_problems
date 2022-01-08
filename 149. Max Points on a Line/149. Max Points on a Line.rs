use std::collections::HashMap;
use std::ops::{ Mul, Sub };
use std::fmt;
#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Frac {
    // represent a faction num / den
    num: i32,
    den: i32,
}
impl Frac {
    fn new(num: i32, den: i32) -> Self {
        assert_ne!(den, 0);
        let mut frac = Frac {
            num,
            den,
        };
        frac.reduce();
        return frac;
    }
    fn reduce(&mut self) {
        if self.num == 0 {
            self.den = 1;
            return;
        }
        let g = gcd(self.num, self.den);
        self.num /= g;
        self.den /= g;
        if self.den < 0 {
            self.num *= -1;
            self.den *= -1;
        }
    }
}
impl From<i32> for Frac {
    fn from(source: i32) -> Frac {
        return Frac::new(source, 1);
    }
}
impl Sub<Frac> for Frac {
    type Output = Frac;
    fn sub(self, rhs: Frac) -> Self::Output {
        return Frac::new(self.num * rhs.den - rhs.num * self.den, self.den * rhs.den);
    }
}
impl Mul<i32> for Frac {
    type Output = Frac;
    fn mul(self, rhs: i32) -> Self::Output {
        return Frac::new(self.num * rhs, self.den);
    }
}
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    } 
    return gcd(b, a % b);
}
fn sqrt(n: i32) -> i32 {
    return (n as f64).sqrt() as i32;
}

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let l = points.len();
        if l <= 2 {
            return l as i32;
        }
        #[derive(Eq, Hash, PartialEq)]
        enum Line {
            Vert(i32),
            Slope(Frac, Frac),
        }
        let mut m: HashMap<Line, i32> = HashMap::new();
        let mut res = 2;
        for i1 in 0..l {
            let p1 = &points[i1];
            for i2 in (i1 + 1)..l {
                let p2 = &points[i2];
                let line = if p1[0] == p2[0] {
                    Line::Vert(p1[0])
                } else {
                    let k = Frac::new(p2[1] - p1[1], p2[0] - p1[0]);
                    let b = Frac::from(p1[1]) - k * p1[0];
                    Line::Slope(k, b)
                };
                if let Some(count) = m.get_mut(&line) {
                    *count += 2;
                    // if a line has n points, the total count will be n * (n - 1)
                    res = res.max((1 + sqrt(1 + 4 * *count)) / 2);
                } else {
                    m.insert(line, 2);
                }
            }
        }
        return res;    
    }
}
