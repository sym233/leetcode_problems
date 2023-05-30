type MultiDimensionalArray = (number | MultiDimensionalArray)[];

var flat = function (arr:  MultiDimensionalArray, n: number):  MultiDimensionalArray {
    if (n === 0) {
        return arr;
    }
    const res = [];

    function dfs(arr: MultiDimensionalArray, n: number) {
        for (const v of arr) {
            if (typeof v === 'number' || n === 0) {
                res.push(v);
            } else {
                dfs(v, n - 1);
            }
        }
    }
    dfs(arr, n);
    return res;
};
