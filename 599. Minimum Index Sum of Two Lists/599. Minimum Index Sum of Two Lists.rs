use std::collections::HashMap;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut m1 = HashMap::<&String, usize>::new();
        for (i, name) in list1.iter().enumerate() {
            m1.insert(name, i);
        }
        let mut shared = Vec::<(usize, &String)>::new();
        for (i, name) in list2.iter().enumerate() {
            if let Some(i2) = m1.get(name) {
                shared.push((i + i2, name));
            }
        }
        let min = shared.iter().min_by_key(|k| k.0).unwrap().0;
        return shared
            .iter()
            .filter(|(i, _)| *i == min)
            .map(|(_, name)| String::from(&name[..]))
            .collect();
    }
}
