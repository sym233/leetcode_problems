/**
 * Definition for Node.
 * class Node {
 *     val: number
 *     children: Node[]
 *     constructor(val?: number, children?: Node[]) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.children = (children===undefined ? [] : children)
 *     }
 * }
 */

function maxDepth(root: Node | null): number {
    if (root) {
        const depths = root.children?.map(maxDepth);
        if (depths?.length) {
            return 1 + Math.max(...depths);
        } else {
            return 1;
        }
    } else {
        return 0;
    }
};
