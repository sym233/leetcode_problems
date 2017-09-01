322. Coin Change

A simple DP.

dp[i] stand for the amount of coins to make up i, initialized to -1.

If i is in coins, assign 1 to dp[i]. If not, let j iterate coins and ensure i > j and dp[i-j] = -1. It means that if we can make up i-j and have coin of j, we can make up i.

Find minimum dp[i-j] which isn't -1 and less than dp[i]-1, then dp[i] = min{dp[i-j]} + 1.

Let i = 1 to amount, we have dp[amount] as solution.

dp[i] = 1 (i in coins)

      | min{dp[i-j]} (j in coins, i > j, dp[i-j] != -1)

      | -1 otherwise

I sorted coins at the start to fasten the search of j.
