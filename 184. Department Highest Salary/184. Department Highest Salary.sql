# Write your MySQL query statement below
SELECT Department.name AS Department, Employee.name AS Employee, salary AS Salary
FROM
    (SELECT MAX(salary) AS s, departmentId
    FROM Employee
        GROUP BY departmentId) AS t1
    JOIN Employee
        USING (departmentId)
    JOIN Department
        ON Employee.departmentId = Department.Id
WHERE Salary = s;
