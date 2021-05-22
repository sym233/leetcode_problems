# @param {Integer} n
# @return {String[][]}
def solve_n_queens(n)
    $res = []
    $n = n
    def dfs(arr)
        len = arr.length
        if len == $n
            $res << arr.map {|i| '.' * i + 'Q' + '.' * ($n - i - 1)}
            return
        end
        for i in 0..$n - 1
            if arr.include?(i)
                next
            end
            flag = false
            for j in 0..len - 1
                if (i - arr[j]).abs == (len - j)
                    flag = true
                    break
                end
            end
            if flag
                next
            end
            dfs(arr + [i])
        end
    end
    dfs([])
    return $res
end
