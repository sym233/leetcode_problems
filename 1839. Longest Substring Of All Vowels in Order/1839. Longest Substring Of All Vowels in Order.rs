impl Solution {
    pub fn longest_beautiful_substring(word: String) -> i32 {
        #[derive(Default)]
        struct SM {
            s: Vec<u8>,
            i: usize,
            begin: bool,
            res: i32,
        }
        impl SM {
            fn new(s: Vec<u8>) -> Self {
                Self {
                    s,
                    ..Default::default()
                }
            }
            fn input(&mut self, b: u8) {
                if self.begin {
                    if b == self.s[self.i] {
                        self.res += 1;
                    } else if self.i + 1 < self.s.len() && b == self.s[self.i + 1] {
                        self.res += 1;
                        self.i += 1;
                    } else {
                        *self = Self::new(std::mem::take(&mut self.s));
                        self.input(b);
                    }
                } else {
                    if b == self.s[0] {
                        self.begin = true;
                        self.res += 1;
                    }
                }
            }
            fn res(&self) -> Option<i32> {
                if self.i + 1 == self.s.len() {
                    Some(self.res)
                } else {
                    None
                }
            }
        }

        let mut sm = SM::new("aeiou".into());

        let mut res = 0;
        for b in word.bytes() {
            sm.input(b);

            if let Some(len) = sm.res() {
                res = res.max(len);
            }
        }
        res
    }
}
