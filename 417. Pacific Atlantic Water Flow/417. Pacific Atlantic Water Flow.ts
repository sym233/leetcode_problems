function pacificAtlantic(heights: number[][]): number[][] {
    const r = heights.length;
    const c = heights[0].length;
    
    const map: number[][] = [];
    for (let i = 0; i < r; i++) {
        map[i] = [];
        for (let j = 0; j < c; j++) {
            map[i][j] = 0;
        }
    }
    
    for (const [ocean, ib, jb] of [[1<<0, 0, 0], [1<<1, r - 1, c - 1]]) {
        const q: [number, number][] = [];
        for (let i = 0; i < r; i++) {
            q.push([i, jb]);
        }
        for (let j = 0; j < c; j++) {
            q.push([ib, j]);
        }
        
        while (q.length) {
            const [i, j] = q.shift();
            map[i][j] |= ocean;
            for (const [di, dj] of [[-1, 0], [1, 0], [0, 1], [0, -1]]) {
                const i2 = i + di;
                const j2 = j + dj;
                if (0 <= i2 && i2 < r && 0 <= j2 && j2 < c && heights[i][j] <= heights[i2][j2] && !(map[i2][j2] & ocean)) {
                    q.push([i2, j2]);
                }
            }
        }
    }
    
    const res = [];
    
    for (let i = 0; i < r; i++) {
        for (let j = 0; j < c; j++) {
            if (map[i][j] === (1 << 0 | 1 << 1)) {
                res.push([i, j]);
            }
        }
    }
    return res;
};
