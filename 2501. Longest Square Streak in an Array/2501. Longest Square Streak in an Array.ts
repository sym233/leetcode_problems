function longestSquareStreak(nums: number[]): number {
    nums.sort((a, b) => a - b);
    const m = {};
    let max = 1;
    for (const num of nums) {
        const prev = Math.sqrt(num);
        if (m[prev]) {
            m[num] = m[prev] + 1;
            max = m[num] > max ? m[num] : max;
        } else {
            m[num] = 1;
        }
    }
    return max > 1 ? max : -1;
};
