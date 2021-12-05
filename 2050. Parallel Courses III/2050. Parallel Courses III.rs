use std::collections::VecDeque;
impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut pre: Vec<usize> = vec![0; n + 1];
        let mut next: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        for rela in relations {
            let a = rela[0] as usize;
            let b = rela[1] as usize;
            pre[b] += 1;
            next[a].push(b);
        }
        let mut free: VecDeque<usize> = VecDeque::new();
        for i in 1..=n {
            if pre[i] == 0 {
                free.push_back(i);
            }
        }
        let mut start: Vec<i32> = vec![0; n + 1];
        let mut latest_end = 0;
        while let Some(course) = free.pop_front() {
            let course_end = start[course] + time[course - 1];
            if latest_end < course_end {
                latest_end = course_end;
            }
            for &next_course in next[course].iter() {
                if start[next_course] < course_end {
                    start[next_course] = course_end;
                }
                pre[next_course] -= 1;
                if pre[next_course] == 0 {
                    free.push_back(next_course);
                }
            }
        }
        return latest_end;
    }
}
