const LEN: usize = 27usize;
const DOT: usize = 27usize;

fn lton(letter: char) -> usize {
    // '\n' -> 0
    // 'a' -> 1
    // ...
    // 'z' -> 26
    // '.' -> DOT
    return match letter {
        '\n' => 0,
        '.' => DOT,
        c => (c as u8 - 'a' as u8 + 1) as usize,
    };
}

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
impl Trie {
    fn new() -> Self {
        return Self {
            t: vec![TrieNode::new()],
        };
    }
    
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
    
    fn search(&self, word: &[char], begin: usize) -> bool {
        if word.len() == 0 {
            return true;
        }
        
        let j = lton(word[0]);
        let curr_node = &self.t[begin];
        
        if j == DOT {
            let mut matched = false;
            for i in 1..=26 {
                if curr_node.child[i] != 0 {
                    matched |= self.search(&word[1..], curr_node.child[i]);
                }
                if matched {
                    return true;
                }
            }
            return false;
        } else if curr_node.child[j] == 0 {
            return false;
        } else {
            return self.search(&word[1..], curr_node.child[j]);
        }
    }
}

struct WordDictionary {
    trie: Trie,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    /** Initialize your data structure here. */
    fn new() -> Self {
        return Self {
            trie: Trie::new(),
        };
    }
    
    /** Adds a word into the data structure. */
    fn add_word(&mut self, word: String) {
        self.trie.insert(word);
    }
    
    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    fn search(&self, word: String) -> bool {
        let word: Vec<char> = word.chars().chain("\n".chars()).collect();
        return self.trie.search(&word[..], 0);
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
