class Solution {
public:
    string multiply(string num1, string num2) {
        // input and result are big endian
        
        const int l1 = num1.length();
        const int l2 = num2.length();
        const int l = l1 + l2;
        
        // little endian
        vector<int> temp(l, 0);
        
        for (int i = 0; i < l1; i++) {
            for (int j = 0; j < l2; j++) {
                int n1 = num1[l1 - i - 1] - '0';
                int n2 = num2[l2 - j - 1] - '0';
                
                temp[i + j] += n1 * n2;
            }
        }
        
        for (int i = 0; i < l; i++) {
            if (temp[i] >= 10) {
                temp[i + 1] += temp[i] / 10;
                temp[i] %= 10;
            }
        }
        
        string res = "";
        bool leadingZero = true;
        for (int i = 0; i < l; i++){
            if (temp[l - i - 1] == 0 && leadingZero) {
                continue;
            } else {
                leadingZero = false;
                res.push_back('0' + temp[l - i - 1]);
            }
        }
        if (res.length() == 0) {
            res.push_back('0');
        }
            
        return res;
            
    }
};
