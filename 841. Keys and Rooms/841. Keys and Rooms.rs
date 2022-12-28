use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let n = rooms.len();
        let mut locked = HashSet::<usize>::from_iter(1..n);
        let mut q = VecDeque::from([0]);
        while let Some(front) = q.pop_front() {
            for &r in &rooms[front] {
                let r = r as usize;
                if locked.remove(&r) {
                    q.push_back(r);
                }
            }
        }
        return locked.is_empty();
    }
}
