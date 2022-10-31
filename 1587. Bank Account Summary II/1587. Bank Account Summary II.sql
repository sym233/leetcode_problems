# Write your MySQL query statement below
SELECT name, balance FROM
    (SELECT account, SUM(amount) as balance FROM Transactions
    GROUP BY account) as b
    JOIN Users
    USING(account)
WHERE balance > 10000;
