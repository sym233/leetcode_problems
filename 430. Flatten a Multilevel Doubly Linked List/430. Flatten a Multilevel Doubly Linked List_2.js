/**
 * // Definition for a Node.
 * function Node(val,prev,next,child) {
 *    this.val = val;
 *    this.prev = prev;
 *    this.next = next;
 *    this.child = child;
 * };
 */

/**
 * @param {Node} head
 * @return {Node}
 */
var flatten = function(head) {    
    let i = head;
    while (i) {
        if (i.child) {
            let j = i.child;
            while (j.next) {
                j = j.next;
            }
            let k = i.next;
            i.next = i.child;
            i.next.prev = i;
            i.child = null;
            if (k) {
                j.next = k;
                j.next.prev = j;
            }
        }
        i = i.next;
    }
    return head;
};
