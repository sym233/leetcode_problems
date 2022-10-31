function maximumBeauty(items: number[][], queries: number[]): number[] {
    items.sort((a, b) => a[0] - b[0]);
    const prefixMax = [];
    for (const item of items) {
        const prev = prefixMax[prefixMax.length - 1];
        if (prev && prev[0] === item[0]) {
            prev[1] = prev[1] > item[1] ? prev[1] : item[1];
        } else {
            const larger = item[1] > (prev?.[1] ?? 0) ? item[1] : prev[1];
            prefixMax.push([item[0], larger]);
        }
    }
    const l = prefixMax.length;
    
    const b = (arr, target) => (s, e) => {
        if (e - s <= 1) {
            return arr[s];
        }
        const m = (s + e) >> 1;
        if (target < arr[m][0]) {
            return b(arr, target)(s, m);
        } else {
            return b(arr, target)(m, e);
        }
    };
    return queries.map(q => {
        const item = b(prefixMax, q)(0, l);
        if (!item || item[0] > q) {
            return 0;
        } else {
            return item[1];
        }
    });
};
