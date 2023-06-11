const SL: usize = 3;
const L: usize = SL * SL;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut row_allow: [S; L] = Default::default();
        let mut col_allow: [S; L] = Default::default();
        let mut subbox_allow: [[S; SL]; SL] = Default::default();
        for (i, j) in (0..L * L).map(rc) {
            if board[i][j] != '.' {
                let n = digit(board[i][j]);
                row_allow[i].del(n);
                col_allow[j].del(n);
                subbox_allow[i / SL][j / SL].del(n);
            }
        }

        let mut d = Dfs {
            b: board,
            r: &mut row_allow,
            c: &mut col_allow,
            s: &mut subbox_allow
        };

        d.dfs(0, 1);
    }
}

#[derive(Clone, Copy)]
struct S(usize);
impl Default for S {
    fn default() -> Self {
        Self((1 << 10) - 1)
    }
}
impl S {
    fn has(&self, n: u8) -> bool {
        self.0 & (1 << n as usize) != 0
    }
    fn del(&mut self, n: u8) {
        self.0 &= !(1 << n as usize)
    }
    fn add(&mut self, n: u8) {
        self.0 |= (1 << n as usize)
    }
}

struct Dfs<'a> {
    b: &'a mut Vec<Vec<char>>,
    r: &'a mut [S; L],
    c: &'a mut [S; L],
    s: &'a mut [[S; SL]; SL],
}

impl<'a> Dfs<'a> {
    fn dfs (&mut self, ind: usize, n: u8) -> bool {
        if ind == L * L {
            return true;
        }
        if n > 9 {
            return false;
        }
        let (i, j) = rc(ind);
        if self.b[i][j] != '.' {
            return self.dfs(ind + 1, 1);
        }
        if self.r[i].has(n)
            && self.c[j].has(n)
            && self.s[i / SL][j / SL].has(n)
        {
            self.r[i].del(n);
            self.c[j].del(n);
            self.s[i / SL][j / SL].del(n);
            self.b[i][j] = ch(n);
            if self.dfs(ind + 1, 1) {
                return true;
            } else {
                self.r[i].add(n);
                self.c[j].add(n);
                self.s[i / SL][j / SL].add(n);
                self.b[i][j] = '.';
            }
        }
        return self.dfs(ind, n + 1);
    }
}

fn rc(ind: usize) -> (usize, usize) {
    (ind / L, ind % L)
}

fn digit(c: char) -> u8 {
    c as u8 - '0' as u8
}

fn ch(d: u8) -> char {
    (d + '0' as u8) as char
}
