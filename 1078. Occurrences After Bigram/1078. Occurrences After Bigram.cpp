class Solution {
public:
    vector<string> findOcurrences(string text, string first, string second) {
        vector<string> words;
        auto bit = text.begin();
        auto eit = text.begin();
        
        for (;;) {
            eit++;
            if (eit == text.end() || *eit == ' ') {
                words.push_back(string(bit, eit));
                if (eit == text.end()) {
                    break;
                } else {
                    bit = eit + 1;
                }
            }
        }
        
        vector<string> res;
        int l = words.size();
        for (int i = 0; i < l - 2; i++) {
            if (words[i] == first && words[i + 1] == second) {
                res.push_back(words[i + 2]);
            }
        }
        
        return res;
    }
};
