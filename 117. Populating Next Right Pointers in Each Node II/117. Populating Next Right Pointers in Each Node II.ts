/**
 * Definition for Node.
 * class Node {
 *     val: number
 *     left: Node | null
 *     right: Node | null
 *     next: Node | null
 *     constructor(val?: number, left?: Node, right?: Node, next?: Node) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */

function connect(root: Node | null): Node | null {
    if (root == null) {
        return null;
    }
    let layer: Node[] = [root];
    let nextLayer: Node[] = [];
    
    for (;;) {
        const l = layer.length;
        if (l == 0) {
            break;
        }
        for (let i = 0; i < l; i++) {
            const node = layer[i];
            if (i + 1 < l) {
                node.next = layer[i + 1];
            }
            if (node.left) {
                nextLayer.push(node.left);
            }
            if (node.right) {
                nextLayer.push(node.right);
            }
        }
        layer = nextLayer;
        nextLayer = [];
    }
    return root;
};
