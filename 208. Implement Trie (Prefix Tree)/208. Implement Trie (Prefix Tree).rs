const LEN: usize = 27usize;

struct TrieNode {
    child: [usize; LEN],
}

impl TrieNode {
    fn new() -> Self {
        return Self {
            child: [0usize; LEN],
        };
    }
}

struct Trie {
    t: Vec<TrieNode>,
}

fn lton(letter: char) -> usize {
    // '\n' -> 0
    // 'a' -> 1
    // ...
    // 'z' -> 26
    if letter == '\n' {
        return 0;
    }
    return (letter as u8 - 'a' as u8 + 1) as usize;
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    /** Initialize your data structure here. */
    fn new() -> Self {
        return Self {
            t: vec![TrieNode::new()],
        };
    }
    
    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut i = 0usize;
        for c in word.chars().chain("\n".chars()) {
            let j = lton(c);
            if self.t[i].child[j] == 0 {
                let l = self.t.len();
                self.t.push(TrieNode::new());
                self.t[i].child[j] = l;
            }
            i = self.t[i].child[j];
        }
    }
    
    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let mut i = 0usize;
        for c in word.chars().chain("\n".chars()) {
            let j = lton(c);
            if self.t[i].child[j] == 0 {
                return false;
            }
            i = self.t[i].child[j];
        }
        return true;
    }
    
    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut i = 0usize;
        for c in prefix.chars() {
            let j = lton(c);
            if self.t[i].child[j] == 0 {
                return false;
            }
            i = self.t[i].child[j];
        }
        return true;
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
