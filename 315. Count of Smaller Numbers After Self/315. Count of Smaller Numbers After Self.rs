use std::default::Default;
use std::ops::Add;
use std::collections::HashMap;

type Func<T> = fn(T, T) -> T;
struct SegTree<T> {
    len: usize,
    data: Vec<T>,
    func: Func<T>,
}

impl<T: Default + Copy> SegTree<T> {
    fn new(data: &[T], func: Func<T>) -> Self {
        let len = data.len();
        let mut st = Self {
            len,
            data: vec![Default::default(); 4 * len],
            func,
        };
        st._build(data, 0, 0, len);
        st
    }
    fn _build(&mut self, data: &[T], node: usize, l: usize, r: usize) {
        if l + 1 == r {
            self.data[node] = data[l];
        } else {
            let mid = (l + r) / 2;
            let ln = node * 2 + 1;
            let rn = node * 2 + 2;
            self._build(data, ln, l, mid);
            self._build(data, rn, mid, r);
            self.data[node] = (self.func)(self.data[ln], self.data[rn]);
        }
    }
    fn query(&self, l: usize, r: usize) -> T {
        self._q(0, 0, self.len, l, r).unwrap()
    }
    fn _q(&self, node: usize, nl: usize, nr: usize, l: usize, r: usize) -> Option<T> {
        // nl, nr:  data[node] covers range [nl..mr]
        if r <= nl || nr <= l {
            return None;
        }
        if l <= nl && nr <= r {
            return Some(self.data[node]);
        }
        let nm = (nl + nr) / 2;
        let cl = self._q(node * 2 + 1, nl, nm, l, r);
        let cr = self._q(node * 2 + 2, nm, nr, l, r);
        if let Some((l, r)) = cl.zip(cr) {
            Some((self.func)(l, r))
        } else {
            cl.or(cr)
        }
    }

    fn update(&mut self, value: T, i: usize) {
        self._u(0, 0, self.len, value, i);
    }
    fn _u(&mut self, node: usize, nl: usize, nr: usize, value: T, i: usize) {
        if i < nl || nr <= i {
            return;
        }
        if nl == i && nl + 1 == nr {
            self.data[node] = value;
            return;
        }
        let nm = (nl + nr) / 2;
        let cl = node * 2 + 1;
        let cr = node * 2 + 2;
        self._u(cl, nl, nm, value, i);
        self._u(cr, nm, nr, value, i);
        self.data[node] = (self.func)(self.data[cl], self.data[cr]);
    }
}

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        const O: usize = 10_001;
        let mut st = SegTree::new(&[0; O * 2], Add::add);
        let mut res = Vec::with_capacity(nums.len());
        let mut counts = HashMap::new();
        for n in nums.into_iter().rev() {
            let n = n as usize + O;
            let q = st.query(0, n);
            res.push(q);
            let count = *counts.entry(n).and_modify(|v| *v += 1).or_insert(1);
            st.update(count, n);
        }
        res.into_iter().rev().collect()
    }
}
