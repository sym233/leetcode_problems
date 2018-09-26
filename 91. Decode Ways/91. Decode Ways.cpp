class Solution {
public:
    int length;
    string str;
    vector<int> cache;
    int beginPos (int b) {
        if (b == length) {
            return 1;
        } else if (b > length){
            return 0;
        }
        if (cache[b] != -1) {
            return cache[b];
        }
        int sum = 0;
        
        string t = str.substr(b, 1);
        int n = stoi(t);
        if (0 < n) {
            sum += beginPos(b + 1);
        }
        
        t = str.substr(b, 2);
        n = stoi(t);
        if (10 <= n && n <= 26) {
            sum += beginPos(b + 2);
        }
        
        cache[b] = sum;
        return sum;
    }
    
    int numDecodings(string s) {
        length = s.length();
        str = s;
        cache = vector<int>(length, -1);
        int res = beginPos(0);
        return res;
    }
};
