# Write your MySQL query statement below

SELECT Department.name AS Department, Employee.name AS Employee, Employee.salary AS Salary FROM
    (SELECT MIN(salary) AS ms, departmentId FROM
        (SELECT 
        ROW_NUMBER() OVER(PARTITION BY departmentId ORDER BY salary DESC) AS r,
        salary, departmentId FROM
            (SELECT DISTINCT salary, departmentId FROM Employee) AS t1
        ) AS t2
        WHERE r <= 3
        GROUP BY departmentId) AS t3
JOIN Employee 
ON t3.departmentId = Employee.departmentId AND Employee.salary >= t3.ms
JOIN Department 
ON Employee.departmentId = Department.id;
