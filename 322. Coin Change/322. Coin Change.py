class Solution(object):
  def coinChange(self, coins, amount):
    """
    :type coins: List[int]
    :type amount: int
    :rtype: int
    """
    if amount == 0:
      return 0
    
    coins.sort()
    
    dp = [-1 for i in range(amount+1)]
    
    for i in range(1, amount+1):
      if i in coins:
        dp[i] = 1
      else:
        for j in coins:
          if j > i:
            break
          else:
            if dp[i - j] != -1 and (dp[i] == -1 or dp[i] > dp[i-j] + 1):
              dp[i] = dp[i-j] + 1
    return dp[amount]
