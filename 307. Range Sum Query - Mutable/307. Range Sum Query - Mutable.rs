use std::ops::Add;
type Pred = fn(i32, i32) -> i32;

struct SegTree {
    len: usize,
    data: Vec<i32>,
    func: Pred,
}

impl SegTree {
    fn new(data: &[i32], func: Pred) -> Self {
        let len = data.len();
        let mut st = Self {
            len,
            data: vec![0; 4 * len],
            func,
        };
        st._build(data, 0, 0, len);
        st
    }
    fn _build(&mut self, data: &[i32], node: usize, l: usize, r: usize) {
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
    fn query(&self, l: usize, r: usize) -> i32 {
        self._q(0, 0, self.len, l, r).unwrap()
    }
    fn _q(&self, node: usize, nl: usize, nr: usize, l: usize, r: usize) -> Option<i32> {
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

    fn update(&mut self, value: i32, i: usize) {
        self._u(0, 0, self.len, value, i);
    }
    fn _u(&mut self, node: usize, nl: usize, nr: usize, value: i32, i: usize) {
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


struct NumArray {
    st: SegTree,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            st: SegTree::new(&nums, Add::add),
        }
    }
    
    fn update(&mut self, index: i32, val: i32) {
        self.st.update(val, index as usize);
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.st.query(left as usize, right as usize + 1)
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */
