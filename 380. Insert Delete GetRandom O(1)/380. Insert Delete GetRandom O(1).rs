use std::collections::HashMap;
use rand::{ Rng, rngs::ThreadRng, thread_rng, seq::SliceRandom };

#[derive(Default)]
struct RandomizedSet {
    m: HashMap<i32, usize>, // element -> its index in self.v
    v: Vec<i32>, // element storage
    r: ThreadRng, // random
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        Self::default()
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if self.m.contains_key(&val) {
            return false;
        }
        self.m.insert(val, self.v.len());
        self.v.push(val);
        true
    }
    
    fn remove(&mut self, val: i32) -> bool {
        let i = if let Some(i) = self.m.get(&val) {
            *i
        } else {
            return false;
        };
        if self.v.len() != i + 1 {
            self.v.swap_remove(i);
            self.m.insert(self.v[i], i);
            self.m.remove(&val);
        } else {
            self.m.remove(&val);
            self.v.pop();
        }
        true
    }
    
    fn get_random(&mut self) -> i32 {
        *self.v.choose(&mut self.r).unwrap()
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
