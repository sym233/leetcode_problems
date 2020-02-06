class Solution {
public:
    string removeDuplicates(string s, int k) {
        struct Item {
            char letter;
            int count;
            Item(char letter) : letter(letter), count(1) {}
        };
        
        deque<Item> que;
        
        
        for (char c : s) {
            if (que.empty()) {
                que.push_back(Item(c));
                continue;
            }
            
            Item& back = que.back();
            if (back.letter == c) {
                back.count++;
                if (back.count == k) {
                    que.pop_back();

                }
                continue;
            }
            
            que.push_back(Item(c));
        }
        
        string res;
        
        for (auto it = que.begin(); it != que.end(); it++) {
            for (int i = 0; i < it->count; i++) {
                res.push_back(it->letter);
            }
        }
        return res;
    }
};

