type Fn = (accum: number, curr: number) => number

function reduce(nums: number[], fn: Fn, init: number): number {
    let val = init;
    for (const num of nums) {
        val = fn(val, num);
    }
    return val;
};
