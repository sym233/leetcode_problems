function deleteGreatestValue(grid: number[][]): number {
    let res = 0;
    const n = grid[0].length;
    for (const line of grid) {
        line.sort((a, b) => b - a);
    }
    for (let i = 0; i < n; i++) {
        let candidates = [];
        for (const line of grid) {
            candidates.push(line[i]);
        }
        res += Math.max(...candidates);
    }
    return res;
    
};
