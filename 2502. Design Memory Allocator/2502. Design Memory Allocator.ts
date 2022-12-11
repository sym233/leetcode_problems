class Allocator {
    private table: {index: number, mid: number, size: number}[];
    constructor(private n: number) {
        this.table = [{index: 0, mid: 0, size: n}];
    }
    allocate(size: number, mID: number): number {
        let allocated = -1;
        const newTable = [];
        for (const b of this.table) {
            if (allocated === -1 && b.mid === 0 && size <= b.size) {
                const newB = {
                    index: b.index,
                    mid: mID,
                    size,
                };
                newTable.push(newB);
                b.index += size;
                b.size -= size;
                allocated = newB.index;
            }
            if (b.size > 0) {
                newTable.push(b);
            }
        }
        if (allocated >= 0) {
            this.table = newTable;
            return allocated;
        }
        return -1;
    }

    free(mID: number): number {
        let freed = 0;
        const newTable = [];
        
        for (const b of this.table) {
            if (b.mid === mID) {
                b.mid = 0;
                freed += b.size;
            }
            const last = newTable[newTable.length - 1];
            if (last && last.mid === 0 && b.mid === 0) {
                last.size += b.size;
            } else {
                newTable.push(b);
            }
        }
        this.table = newTable;
        return freed;
    }
}

/**
 * Your Allocator object will be instantiated and called as such:
 * var obj = new Allocator(n)
 * var param_1 = obj.allocate(size,mID)
 * var param_2 = obj.free(mID)
 */
