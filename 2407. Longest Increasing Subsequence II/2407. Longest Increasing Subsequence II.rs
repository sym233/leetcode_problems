use std::default::Default;
use std::clone::Clone;

type Func<T> = fn(T, T) -> T;
struct SegTree<T> {
    len: usize,
    data: Vec<T>,
    func: Func<T>,
}

impl<T: Default + Clone> SegTree<T> {
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
            self.data[node] = data[l].clone();
        } else {
            let mid = (l + r) / 2;
            let ln = node * 2 + 1;
            let rn = node * 2 + 2;
            self._build(data, ln, l, mid);
            self._build(data, rn, mid, r);
            self.data[node] = (self.func)(self.data[ln].clone(), self.data[rn].clone());
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
            return Some(self.data[node].clone());
        }
        let nm = (nl + nr) / 2;
        let cl = self._q(node * 2 + 1, nl, nm, l, r);
        let cr = self._q(node * 2 + 2, nm, nr, l, r);
        if let Some((l, r)) = cl.as_ref().zip(cr.as_ref()) {
            Some((self.func)(l.clone(), r.clone()))
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
        self._u(cl, nl, nm, value.clone(), i);
        self._u(cr, nm, nr, value, i);
        self.data[node] = (self.func)(self.data[cl].clone(), self.data[cr].clone());
    }
}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>, k: i32) -> i32 {
        const OFFSET: usize = 100_001;
        let mut st = SegTree::new(&[0; OFFSET * 2], i32::max);
        let k = k as usize;
        let mut res = 0;
        for n in nums {
            let n = n as usize + OFFSET;
            let len = st.query(n - k, n) + 1;
            res = res.max(len);
            st.update(len, n);
        }
        res
    }
}
