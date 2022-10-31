# Write your MySQL query statement below
SELECT DISTINCT l1.num as ConsecutiveNums FROM 
    Logs AS l1 JOIN Logs AS l2
        ON l1.id + 1 = l2.id AND l1.num = l2.num
    JOIN Logs AS l3
        ON l1.id + 2 = l3.id AND l1.num = l3.num;
