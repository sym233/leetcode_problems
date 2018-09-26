class Solution {
public:
    int modulo = 1e9 + 7;
    int length;
    string str;
    vector<long long> cache;
    long long beginPos (int b) {
        
        if (b == length) {
            return 1;
        } else if (b > length){
            return 0;
        }
        if (cache[b] != -1) {
            return cache[b];
        }
        long long sum = 0;
        
        if (str[b] == '*'){
            sum += 9 * beginPos(b + 1);
        } else {
            string t = str.substr(b, 1);
            int n = stoi(t);
            if (0 < n) {
                sum += beginPos(b + 1);
            }
        }
        
        if (b + 1 < length) {
            if (str[b] == '*') {
                if (str[b + 1] == '*') {
                    // ** -> 11 ~ 19 | 21 ~ 26
                    // * can't be treated as '0'
                    sum += 15 * beginPos(b + 2);
                } else if (str[b + 1] <= '6') {
                    // *[0 - 6] -> 1[0 - 6], 2[0 - 6]
                    sum += 2 * beginPos(b + 2);
                } else {
                    // *[7-9] -> 1[7-9]
                    sum += beginPos(b + 2);
                }
            } else {
                if (str[b + 1] == '*') {
                    if (str[b] == '0') {
                        // 0* -> none
                        // pass
                    } else if (str[b] == '1') {
                        // 1* -> 1[1 - 9]
                        sum += 9 * beginPos(b + 2);
                    } else if (str[b] == '2') {
                        // 2* -> 2[1 - 6]
                        sum += 6 * beginPos(b + 2);
                    } else {
                        // [3 - 7]* -> none
                        // pass
                    }
                } else {
                    // no '*'
                    string t = str.substr(b, 2);
                    int n = stoi(t);
                    if (10 <= n && n <= 26) {
                        sum += beginPos(b + 2);
                    }
                }
            }
        }

        sum %= modulo;
        cache[b] = sum;
        return sum;
    }
    
    int numDecodings(string s) {
        length = s.length();
        str = s;
        cache = vector<long long>(length, -1);
        int res = static_cast<int>(beginPos(0));
        
        return res;
    }
};
