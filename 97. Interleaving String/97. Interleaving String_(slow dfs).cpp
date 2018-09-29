class Solution {
public:
    string st1 = "";
    string st2 = "";
    string st3 = "";
    int l1 = 0;
    int l2 = 0;
    int l3 = 0;
    bool res = false;
    bool isInterleave(string s1, string s2, string s3) {
        l1 = s1.length();
        l2 = s2.length();
        l3 = s3.length();
        if (l1 + l2 != l3) {
            return false;
        }
        
        st1 = s1;
        st2 = s2;
        st3 = s3;
        res = false;
        
        test(0, 0, 0);
        return res;
        
    }
    
    void test(int p1, int p2, int p3) {
        if (res) {
            return;
        }
        if (p3 == l3) {
            res = true;
            return;
        }
        if (p1 < l1 && st1[p1] == st3[p3]) {
            test(p1 + 1, p2, p3 + 1);
        }
        if (p2 < l2 && st2[p2] == st3[p3]) {
            test(p1, p2 + 1, p3 + 1);
        }
    }
};
