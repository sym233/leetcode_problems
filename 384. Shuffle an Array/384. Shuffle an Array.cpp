class Solution {
private:
    vector<int> n, o;
    int l;
    int randInt(int end) {
        // generate a random int in [0, end)
        int r = rand();
        double d = r / (RAND_MAX + 1.0);
        return static_cast<int>(d * end);
    }
public:
    Solution(vector<int>& nums) {
        n = nums;
        o = nums;
        l = n.size();
    }
    
    /** Resets the array to its original configuration and return it. */
    vector<int> reset() {
        n = o;
        return n;
    }
    
    /** Returns a random shuffling of the array. */
    vector<int> shuffle() {
        for (int i = 0; i < l; i++) {
            int r = i + randInt(l - i);
            swap(n[i], n[r]);
        }
        return n;
    }
};

/**
 * Your Solution object will be instantiated and called as such:
 * Solution* obj = new Solution(nums);
 * vector<int> param_1 = obj->reset();
 * vector<int> param_2 = obj->shuffle();
 */
