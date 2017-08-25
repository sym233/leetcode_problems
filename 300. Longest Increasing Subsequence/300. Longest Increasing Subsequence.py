class Solution(object):
    def lengthOfLIS(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        maxl = 1;
        l = len(nums)
        if l <= 1:
            return l
        dp = [1 for i in range(l)]
        
        for i in range(1, l):
            for j in range(i):
                if nums[j] < nums[i]:
                    if dp[i] < dp[j] + 1:
                        dp[i] = dp[j] + 1
            if dp[i] > maxl:
                maxl = dp[i]
        return maxl
