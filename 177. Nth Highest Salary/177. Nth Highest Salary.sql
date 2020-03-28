CREATE FUNCTION getNthHighestSalary(N INT) RETURNS INT
BEGIN
  DECLARE ofs INT;
  SET ofs = N - 1;
  RETURN (
    # Write your MySQL query statement below.
    SELECT 
      IFNULL ((
        SELECT DISTINCT Salary
        FROM Employee
        ORDER BY Salary DESC
        LIMIT ofs, 1
      ), NULL) 
    AS SecondHighestSalary
  );
END
