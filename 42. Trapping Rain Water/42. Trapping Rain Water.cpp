class Solution {
public:
    int trap(vector<int>& height) {
        int water = 0;
        height.push_back(0);
        
        int len = height.size();
        
        queue<int> peaks;
        
        for (int i = 0; i < len; i++) {
            if ((i == 0 && height[i] >= height[i + 1]) || (i != 0) && height[i - 1] < height[i] && height[i] >= height[i + 1]) {
                peaks.push(i);
            }
        }

        while (peaks.size() > 1) {
            int l = peaks.size();
            
            int leftPeak = peaks.front();
            peaks.pop();
            int rightPeak;
            for (int i = 0; i < l - 1; i++) {
                // loop l - i times, the first has been popped
                
                rightPeak = peaks.front();
                peaks.pop();
                
                int lowerPeak = height[leftPeak] < height[rightPeak] ? height[leftPeak] : height[rightPeak];
                int higherPeakIndex = height[leftPeak] > height[rightPeak] ? leftPeak : rightPeak;
                
                // fill water from leftPeak to rightPeak
                for (int j = leftPeak + 1; j < rightPeak; j++) {
                    if (height[j] < lowerPeak) {
                        water += lowerPeak - height[j];
                        height[j] = lowerPeak;
                    }
                }
                leftPeak = rightPeak;
                
                if (peaks.back() != higherPeakIndex) {
                    peaks.push(higherPeakIndex);
                }
            }
        }
        return water;
    }
};
