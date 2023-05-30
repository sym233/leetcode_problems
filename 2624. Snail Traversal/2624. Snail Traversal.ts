declare global {
  interface Array<T> {
    snail(rowsCount: number, colsCount: number): number[][];
  }
}

Array.prototype.snail = function(rowsCount: number, colsCount: number): number[][] {
    if (this.length !== rowsCount * colsCount) {
        return [];
    }
    const it = this[Symbol.iterator]();
    const res = [];
    for (let c = 0; c < colsCount; c++) {
        if (c % 2 == 0) {
            for (let r = 0; r < rowsCount; r++) {
                if (res[r] === undefined) {
                    res[r] = [];
                }
                res[r][c] = it.next().value;
            }
        } else {
            for (let r = rowsCount - 1; r >= 0; r--) {
                res[r][c] = it.next().value;
            }
        }
    }
    return res;
}

/**
 * const arr = [1,2,3,4];
 * arr.snail(1,4); // [[1,2,3,4]]
 */
