impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        struct Ptn {
            ch: u8,
            aster: bool,
        }
        impl Ptn {
            fn new(ch: u8) -> Self {
                Self { ch, aster: false }
            }
        }

        let mut ptns: Vec<Ptn> = vec![];
        for b in p.bytes() {
            match b {
                b'*' => ptns.last_mut().unwrap().aster = true,
                b @ _ => ptns.push(Ptn::new(b)),
            }
        }

        let s = s.as_bytes();
        let sl = s.len();
        let pl = ptns.len();

        let mut dp = vec![vec![false; pl + 1]; sl + 1];
        dp[0][0] = true;

        for i in 0..=sl {
            for j in 0..pl {
                if !dp[i][j] {
                    continue;
                }
                match ptns[j] {
                    Ptn { ch, aster: false } => {
                        if i < sl && (ch == b'.' ||  ch == s[i]) {
                            dp[i + 1][j + 1] = true;
                        }
                    },
                    Ptn { ch: b'.', aster: true } => {
                        for i2 in i..=sl {
                            dp[i2][j + 1] = true;
                        }
                    },
                    Ptn { ch, aster: true } => {
                        dp[i][j + 1] = true;
                        for i2 in i..sl {
                            if s[i2] == ch {
                                dp[i2 + 1][j + 1] = true;
                            } else {
                                break;
                            }
                        }
                    },
                }
            }
        }
        dp[sl][pl]
    }
}
