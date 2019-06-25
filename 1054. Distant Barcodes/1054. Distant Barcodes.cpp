class Solution {
public:
    vector<int> rearrangeBarcodes(vector<int>& barcodes) {
        int l = barcodes.size();
        unordered_map<int, int> counts;

        for (int& num : barcodes) {
            counts[num]++;
        }
        
        multimap<int, int> sorted;
        for (unordered_map<int, int>::iterator it = counts.begin(); 
             it != counts.end();
             it++) {
            int num = it->first;
            int times = it->second;
            sorted.insert({times, num});
        }
        
        vector<int> res(l, 0);
        
        int i = 0;
        for (auto it = sorted.rbegin();
             it != sorted.rend();
             it++) {
            int times = it->first;
            int num = it->second;
            for (int j = 0; j < times; j++) {
                if (i >= l) {
                    i -= l;
                }
                if (res[i] != 0) {
                    i++;
                }
                // cout << num << " @ " << i << endl;
                res[i] = num;
                i += 2;
            }
        }
        return res;
    }
};
