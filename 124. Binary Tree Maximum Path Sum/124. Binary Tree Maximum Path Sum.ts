/**
 * Definition for a binary tree node.
 * class TreeNode {
 *     val: number
 *     left: TreeNode | null
 *     right: TreeNode | null
 *     constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *     }
 * }
 */

function maxPathSum(root: TreeNode | null): number {
    let max = -Infinity;
    function path(node: TreeNode | null): number {
        if (!node) {
            return 0;
        }
        const l = path(node.left);
        const r = path(node.right);
        const localMax = Math.max(node.val, l + node.val, node.val + r, l + node.val + r);
        max = localMax > max ? localMax : max;
        const subPath = Math.max(node.val, l + node.val, node.val + r);
        return subPath;
    }

    path(root);
    return max;
};
