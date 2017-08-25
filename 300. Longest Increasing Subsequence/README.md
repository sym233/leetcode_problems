300. Longest Increasing Subsequence

Dynamic Programming

dp[i] stores the length of the longest subsequence ending with nums[i]

dp[0] = 1

dp[i] = max{dp[j] where j < i and num[j] < num[i]} + 1

time complexity: O(n^2)

--------------------------------------

I will commit another solution with binary search & dp in O(nlogn) time someday (gugu)
