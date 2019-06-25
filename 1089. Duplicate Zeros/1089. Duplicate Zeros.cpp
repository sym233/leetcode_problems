class Solution {
public:
    void duplicateZeros(vector<int>& arr) {
        int l = arr.size();
        vector<int> newArr;
        for (int& a : arr) {
            if (newArr.size() < l) {
                if (a == 0) {
                    newArr.push_back(0);
                    newArr.push_back(0);
                } else {
                    newArr.push_back(a);
                }
            } else {
                break;
            }
        }
        
        if (newArr.size() > l) {
            newArr.pop_back();
        }
        arr = newArr;
    }
};
