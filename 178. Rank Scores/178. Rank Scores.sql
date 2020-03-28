/* Write your T-SQL query statement below */
#  Leetcode does not support mysql 8
SELECT Score, DENSE_RANK()
OVER (ORDER BY Score DESC) AS Rank
FROM Scores;
