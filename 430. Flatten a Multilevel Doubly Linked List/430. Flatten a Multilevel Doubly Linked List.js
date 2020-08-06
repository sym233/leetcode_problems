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
    if (!head) {
        return null;
    }
    const fakeHead = new Node(0, null, null, null);
    let tail = fakeHead;
    const stack = [head];
    
    while (stack.length) {
        const top = stack.pop();
        if (top.next) {
            top.next.prev = null;
            stack.push(top.next);
            top.next = null;
        }
        if (top.child) {
            stack.push(top.child);
            top.child = null;
        }
        
        tail.next = top;
        top.prev = tail;
        tail = tail.next;
    }
    
    if (fakeHead.next) {
        fakeHead.next.prev = null;
    }
    return fakeHead.next;
};
