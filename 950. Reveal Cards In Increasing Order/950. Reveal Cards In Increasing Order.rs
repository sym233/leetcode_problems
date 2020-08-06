impl Solution {
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        use std::collections::VecDeque;
        
        let l = deck.len();
        let mut deq: VecDeque<i32> = VecDeque::with_capacity(l);
        
        let mut deck = deck;
        deck.sort_by(|a, b| b.cmp(a));
        
        for n in deck {
            if let Some(top) = deq.pop_back() {
                deq.push_front(top);
            }
            deq.push_front(n);
        }
        
        return deq.iter().map(|&n| n).collect();
    }
}
